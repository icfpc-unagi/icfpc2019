package api

import (
	"context"
	"fmt"
	"html"
	"io/ioutil"
	"net/http"

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
	if err := insertProblemHandler(ctx, req, resp); err != nil {
		return err
	}
	return nil
}

func insertProblemHandler(
	ctx context.Context, apiReq *pb.Api_Request, apiResp *pb.Api_Response,
) error {
	req := apiReq.GetInsertProblem()
	if req == nil {
		return nil
	}

	if req.GetProblemName() == "" {
		return errors.New("problem_name is missing")
	}
	if req.GetProblemData() == nil || len(req.GetProblemData()) == 0 {
		return errors.New("problem_data is missing")
	}

	res, err := db.Execute(ctx,
		"INSERT problems(problem_name) VALUES(?)", req.GetProblemName())
	if err != nil {
		return err
	}
	id, err := res.LastInsertId()
	if err != nil {
		return errors.WithStack(err)
	}
	res, err = db.Execute(ctx,
		"INSERT problem_data(problem_id, problem_data_blob) VALUES(?, ?)",
		id, req.GetProblemData())
	if err != nil {
		return err
	}
	apiResp.InsertProblem = &pb.Api_Response_InsertProblem{ProblemId: id}
	return nil
}
