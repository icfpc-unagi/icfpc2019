package handler

import (
	"context"
	"fmt"
	"net/http"
	"strconv"

	"github.com/imos/icfpc2019/go/util/db"
	"google.golang.org/appengine"
	"google.golang.org/appengine/log"
)

func init() {
	http.HandleFunc("/problem_image", func(w http.ResponseWriter, r *http.Request) {
		ctx := appengine.NewContext(r)
		output, err := programImageHandler(ctx, r)
		if err != nil {
			log.Errorf(ctx, "%+v", err)
			http.Error(w, fmt.Sprintf("%+v", err), 500)
			return
		}
		w.Header().Set("Content-Type", "image/png")
		w.Header().Set("Cache-Control", "public")
		_, err = w.Write(output)
		if err != nil {
			log.Errorf(ctx, "%+v", err)
		}
	})
}

func programImageHandler(ctx context.Context, r *http.Request) ([]byte, error) {
	problemID, err := strconv.ParseInt(r.FormValue("problem_id"), 10, 64)
	if err != nil {
		return nil, err
	}
	problems := []struct {
		ProblemDataImage []byte `db:"problem_data_image"`
	}{}
	if err := db.Select(ctx, &problems, `
		SELECT
			problem_data_image
		FROM problem_data
		WHERE problem_id = ?
		LIMIT 1`, problemID); err != nil {
		return nil, err
	}
	if len(problems) == 0 {
		return nil, err
	}
	return problems[0].ProblemDataImage, nil
}
