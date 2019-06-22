package handler

import (
	"context"
	"fmt"
	"net/http"
	"strings"

	"github.com/imos/icfpc2019/go/util/db"
)

func init() {
	registerHandler("/problems/", problemsHandler)
}

func problemsHandler(ctx context.Context, r *http.Request) (HTML, error) {
	problems := []struct {
		ProblemID      int64  `db:"problem_id"`
		ProblemName    string `db:"problem_name"`
		ProblemCreated string `db:"problem_created"`
	}{}
	if err := db.Select(ctx, &problems,
		`SELECT problem_id, problem_name, problem_created `+
			`FROM problems ORDER BY problem_name`); err != nil {
		return "", err
	}
	output := HTML(
		`<table class="table table-clickable">` +
			`<thead><tr><td>ID</td><td>Name</td><td>Created</td></thead>` +
			`<tbody>`)

	for _, problem := range problems {
		img := strings.Replace(problem.ProblemName, ".desc", ".png", 1)
		output += "<tr><td>" +
			Escape(fmt.Sprintf("%d", problem.ProblemID)) +
			`</td><td><img src="/static/` + Escape(img) + `">` +
			Escape(problem.ProblemName) +
			"</a></td><td>" +
			Escape(problem.ProblemCreated) +
			"</td></tr>"
	}
	output += `</tbody></table>`
	return output, nil
}
