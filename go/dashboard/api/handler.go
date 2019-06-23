package api

import (
	"context"
	"fmt"
	"html"
	"io/ioutil"
	"net/http"
	"strings"

	"github.com/golang/protobuf/proto"
	"github.com/pkg/errors"
	"google.golang.org/appengine"
	"google.golang.org/appengine/log"

	"github.com/imos/icfpc2019/go/util/db"
	"github.com/imos/icfpc2019/go/util/pb"
)

func init() {
	http.HandleFunc("/api/", func(w http.ResponseWriter, r *http.Request) {
		if err := handler(w, r); err != nil {
			log.Errorf(appengine.NewContext(r), "%+v", err)
			http.Error(w, fmt.Sprintf("%+v", err), 500)
			return
		}
	})
}

func handler(w http.ResponseWriter, r *http.Request) error {
	ctx := appengine.NewContext(r)
	req := &pb.Api_Request{}
	resp := &pb.Api_Response{
		Context: &pb.Api_Response_Context{},
	}
	if r.Body != nil {
		defer r.Body.Close()
	}
	if r.Header.Get("Content-Type") == "application/protobuf" {
		if r.Body == nil {
			return errors.New("request body is missing")
		} else if buf, err := ioutil.ReadAll(r.Body); err != nil {
			return errors.New("failed to read to read the request body")
		} else if err := proto.Unmarshal(buf, req); err != nil {
			return errors.Errorf("failed to parse request: %s", err)
		}
	} else {
		r.ParseForm()
		if err := proto.UnmarshalText(
			r.PostForm.Get("request"), req); err != nil {
			resp.Context.ErrorMessages = append(
				resp.Context.GetErrorMessages(), err.Error())
		}
	}
	if err := apiHandler(ctx, req, resp); err != nil {
		return err
	}
	if r.Header.Get("Content-Type") == "application/protobuf" {
		buf, err := proto.Marshal(resp)
		if err != nil {
			return errors.Errorf("failed to encode response: %s", err)
		}
		w.Header().Set("Content-Type", "application/protobuf")
		w.Write(buf)
	} else {
		w.Write([]byte(
			fmt.Sprintf(`
<html><body>
<form action=. method=POST>
<textarea name=request style="width:100%%;height:20em;">%s</textarea>
<pre>%s</pre>
<input type="submit" value="Submit">
</form><body>`,
				html.EscapeString(proto.MarshalTextString(req)),
				html.EscapeString(proto.MarshalTextString(resp)),
			)))
	}
	return nil
}

func apiHandler(
	ctx context.Context, req *pb.Api_Request, resp *pb.Api_Response) error {
	defer func() {
		if proto.Size(resp) < 100000 {
			log.Debugf(ctx, "Response: %s", proto.MarshalTextString(resp))
		}
	}()
	if proto.Size(req) < 100000 {
		log.Debugf(ctx, "Request: %s", proto.MarshalTextString(req))
	}
	if err := insertProblemHandler(ctx, req, resp); err != nil {
		return err
	}
	if err := insertProgramHandler(ctx, req, resp); err != nil {
		return err
	}
	if err := acquireSolutionHandler(ctx, req, resp); err != nil {
		return err
	}
	if err := updateSolutionHandler(ctx, req, resp); err != nil {
		return err
	}
	if err := insertSolutionHandler(ctx, req, resp); err != nil {
		return err
	}
	if err := acquireProblemExtraHandler(ctx, req, resp); err != nil {
		return err
	}
	if err := updateProblemExtraHandler(ctx, req, resp); err != nil {
		return err
	}
	if err := acquireSolutionExtraHandler(ctx, req, resp); err != nil {
		return err
	}
	if err := updateSolutionExtraHandler(ctx, req, resp); err != nil {
		return err
	}
	if err := extendSolutionHandler(ctx, req, resp); err != nil {
		return err
	}
	return nil
}

func insertProgramHandler(
	ctx context.Context, apiReq *pb.Api_Request, apiResp *pb.Api_Response,
) error {
	req := apiReq.GetInsertProgram()
	resp := &pb.Api_Response_InsertProgram{}
	if req == nil {
		return nil
	}

	tx, err := db.DB().BeginTxx(ctx, nil)
	if err != nil {
		return errors.WithStack(err)
	}
	if err := func() error {
		result, err := tx.ExecContext(
			ctx, "INSERT programs(program_name, program_code) VALUES(?, ?)",
			req.GetProgramName(), req.GetProgramCode())
		if err != nil {
			return errors.WithStack(err)
		}
		id, err := result.LastInsertId()
		if err != nil {
			return errors.WithStack(err)
		}
		resp.ProgramId = id
		for _, booster := range strings.Split(req.GetProgramBoosters(), ",") {
			_, err = tx.ExecContext(ctx, `
				INSERT IGNORE INTO solutions(
					program_id, solution_booster, problem_id, solution_lock)
				SELECT
					? AS program_id,
					? AS solution_booster,
					problem_id,
					NOW() - INTERVAL (RAND() + 1) * 24 * 60 * 60 SECOND
						AS solution_lock
				FROM problems`,
				id, booster)
			if err != nil {
				return errors.WithStack(err)
			}
		}
		return nil
	}(); err != nil {
		tx.Rollback()
		return err
	}
	apiResp.InsertProgram = resp
	return tx.Commit()
}
