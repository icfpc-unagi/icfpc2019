package handler

import (
	"context"
	"fmt"
	"net/http"
	"strconv"

	"github.com/imos/icfpc2019/go/util/db"
)

func init() {
	registerHandler("/program", programHandler)
}

func programHandler(ctx context.Context, r *http.Request) (HTML, error) {
	programID, err := strconv.ParseInt(r.FormValue("program_id"), 10, 64)
	if err != nil {
		return "", err
	}
	problems := []struct {
		ProblemID        int64   `db:"problem_id"`
		ProblemName      string  `db:"problem_name"`
		SolutionID       *int64  `db:"solution_id"`
		SolutionScore    *int64  `db:"solution_score"`
		SolutionModified *string `db:"solution_modified"`
	}{}
	if err := db.Select(ctx, &problems, `
		SELECT
			problem_id,
			problem_name,
			MAX(solution_id) AS solution_id,
			solution_score,
			MAX(solution_modified) AS solution_modified
		FROM (
			SELECT
				problem_id,
				problem_name,
				MIN(solution_score) AS solution_score
			FROM
				programs
				NATURAL LEFT JOIN problems
				NATURAL LEFT JOIN solutions
			WHERE program_id = ?
			GROUP BY problem_id
			ORDER BY solution_score DESC) AS t
			NATURAL LEFT JOIN solutions
		GROUP BY problem_id, solution_score
		ORDER BY problem_name`, programID); err != nil {
		return "", err
	}
	output := HTML(
		`<table class="table table-clickable">` +
			`<thead><tr><td>Name</td><td>Score</td><td>Modified</td></thead>` +
			`<tbody>`)
	for _, problem := range problems {
		score := "-"
		if problem.SolutionScore != nil {
			if *problem.SolutionScore >= 100000000 {
				score = "invalid"
			} else {
				score = fmt.Sprintf("%d", *problem.SolutionScore)
			}
		}
		modified := "-"
		if problem.SolutionModified != nil {
			modified = *problem.SolutionModified
		}
		output += `<tr><td><img src="/problem_image?problem_id=` + Escape(fmt.Sprintf("%d", problem.ProblemID)) + `">` +
			Escape(problem.ProblemName) +
			`</td><td><a href="/solution?solution_id=` + Escape(fmt.Sprintf("%d", *problem.SolutionID)) +
			`"><img src="/solution_image?solution_id=` + Escape(fmt.Sprintf("%d", *problem.SolutionID)) + `">` +
			Escape(score) +
			"</a></td><td>" +
			Escape(modified) +
			"</td></tr>"
	}
	output += `</tbody></table>`
	return output, nil
}
