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
	program := struct{
		ProgramName    string `db:"program_name"`
		ProgramCode    string `db:"program_code"`
	}{}
	if err := db.Row(ctx, &program,
		`SELECT
			program_name,
			program_code
		FROM programs
		WHERE program_id = ?
		LIMIT 1`, programID); err != nil {
		return "", err
	}
	problems := []struct {
		ProblemID        int64   `db:"problem_id"`
		ProblemName      string  `db:"problem_name"`
		SolutionID       *int64  `db:"solution_id"`
		SolutionScore    *int64  `db:"solution_score"`
		SolutionBooster  *string `db:"solution_booster"`
		SolutionModified *string `db:"solution_modified"`
	}{}
	if err := db.Select(ctx, &problems, `
		SELECT
			problem_id,
			problem_name,
			solution_booster,
			MAX(solution_id) AS solution_id,
			solution_score,
			MAX(solution_modified) AS solution_modified
		FROM (
			SELECT
				problem_id,
				solution_booster,
				problem_name,
				MIN(solution_score) AS solution_score
			FROM
				programs
				NATURAL LEFT JOIN problems
				NATURAL LEFT JOIN solutions
			WHERE program_id = ?
			GROUP BY problem_id, solution_booster
			ORDER BY solution_score DESC) AS t
			NATURAL LEFT JOIN solutions
		GROUP BY problem_id, solution_booster, solution_score
		ORDER BY problem_name, solution_booster`, programID); err != nil {
		return "", err
	}
	output := HTML(
		`<h2 style="display:inline-block;margin-right:10px">` + Escape(program.ProgramName) + `</h2>` +
		`<code style="border:solid 1px silver;border-radius:3px;background:white;padding:2px">` + Escape(program.ProgramCode) + `</code>` +
		`<table class="table table-clickable">` +
			`<thead><tr><td>Name</td><td>Booster</td>` +
			`<td>Score</td><td width="300">Image</td><td>Modified</td></thead>` +
			`<tbody>`)
	for _, problem := range problems {
		booster := HTML("None")
		if problem.SolutionBooster != nil && *problem.SolutionBooster != "" {
			booster = Escape(*problem.SolutionBooster)
		}
		score := HTML("-")
		image := HTML("")
		dataUrl := HTML("")
		if problem.SolutionScore != nil {
			if *problem.SolutionScore >= 100000000 {
				score = "invalid"
			} else {
				score = Escape(fmt.Sprintf("%d", *problem.SolutionScore))
			}
			image = `<img src="/solution_image?solution_id=` + Escape(fmt.Sprintf("%d", *problem.SolutionID)) + `" style="max-width:300px;height:auto">`
			dataUrl = ` data-url="/solution?solution_id=` + Escape(fmt.Sprintf("%d", *problem.SolutionID)) + `"`
		}
		modified := "-"
		if problem.SolutionModified != nil {
			modified = *problem.SolutionModified
		}
		output += `<tr` + dataUrl + `><td>` +
			Escape(problem.ProblemName) +
			"</td><td>" +
			booster +
			"</td><td>" +
			score +
			"</td><td>" +
			image +
			"</td><td>" +
			Escape(modified) +
			"</td></tr>"
	}
	output += `</tbody></table>`
	return output, nil
}
