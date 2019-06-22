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
	http.HandleFunc("/solution_image", func(w http.ResponseWriter, r *http.Request) {
		ctx := appengine.NewContext(r)
		output, err := solutionImageHandler(ctx, r)
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

func solutionImageHandler(ctx context.Context, r *http.Request) ([]byte, error) {
	solutionID, err := strconv.ParseInt(r.FormValue("solution_id"), 10, 64)
	if err != nil {
		return nil, err
	}
	solutions := []struct {
		SolutionDataImage []byte `db:"solution_data_image"`
	}{}
	if err := db.Select(ctx, &solutions, `
		SELECT
			solution_data_image
		FROM solution_data
		WHERE solution_id = ?
		LIMIT 1`, solutionID); err != nil {
		return nil, err
	}
	if len(solutions) == 0 {
		return nil, err
	}
	return solutions[0].SolutionDataImage, nil
}
