package handler

import (
	"context"
	"fmt"
	"net/http"

	"github.com/imos/icfpc2019/go/util/db"
)

func init() {
	registerHandler("/status/", statusHandler)
}

func statusHandler(ctx context.Context, r *http.Request) (HTML, error) {
	status := struct {
		Waiting int64 `db:"waiting"`
		Running int64 `db:"running"`
	}{}
	if err := db.Row(ctx, &status, `
		SELECT
			(SELECT COUNT(*) FROM solutions
			WHERE solution_lock < NOW()) AS waiting,
			(SELECT COUNT(*) FROM solutions
			WHERE solution_lock >= NOW()) AS running`); err != nil {
		return "", err
	}
	var output HTMLBuffer
	output.WriteHTML("<h1>Server Status</h1>")
	output.WriteHTML("<ul><li>")
	output.WriteString(fmt.Sprintf("Waiting jobs ... %d", status.Waiting))
	output.WriteHTML("</li><li>")
	output.WriteString(fmt.Sprintf("Running jobs ... %d", status.Running))
	output.WriteHTML("</li></ul>")
	return output.HTML(), nil
}
