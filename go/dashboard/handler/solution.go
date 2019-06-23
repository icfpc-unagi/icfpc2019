package handler

import (
	"bytes"
	"context"
	"html/template"
	"net/http"
	"strconv"

	"github.com/imos/icfpc2019/go/util/db"
)

func init() {
	registerHandler("/solution", solutionHandler)
}

var tmpl = template.Must(template.New("solution").Parse(`
<div style="width:100%">
	<table class="table" style="width:500px;margin:auto" align="center">
		<thead><tr><td>ID</td><td>Program</td><td>Problem</td><td>Booster</td><td>Score</td><td>Modified</td></tr></thead>
		<tbody>
			<tr>
				<td>{{.SolutionID}}</td>
				<td>{{.ProgramName}} ({{.ProgramID}})</td>
				<td>{{.ProblemName}} ({{.ProblemID}})</td>
				<td>{{.SolutionBooster}}</td>
				<td>{{.SolutionScore}}</td>
				<td>{{.SolutionModified}}</td>
			</tr>
		</tbody>
	</table>

	<a href="#output">Output</a> <a href="#error">Error</a>
	<p>{{.SolutionDescription}}</p>

	<div style="text-align:center"><img src="/solution_image?solution_id={{.SolutionID}}"></div>

	<h3><a name="output">Output:</a></h3>
	<pre>{{.SolutionDataBlob}}</pre>

	<h3><a name="error">Error:</a></h3>
	<pre>{{.SolutionDataError}}</pre>
</div>
`))

func solutionHandler(ctx context.Context, r *http.Request) (HTML, error) {
	solutionID, err := strconv.ParseInt(r.FormValue("solution_id"), 10, 64)
	if err != nil {
		return "", err
	}
	solution := struct {
		SolutionID          int64   `db:"solution_id"`
		SolutionBooster     string  `db:"solution_booster"`
		SolutionScore       *int64  `db:"solution_score"`
		SolutionModified    *string `db:"solution_modified"`
		SolutionDescription string  `db:"solution_description"`
		SolutionDataBlob    string  `db:"solution_data_blob"`
		SolutionDataError   string  `db:"solution_data_error"`
		ProgramID           int64   `db:"program_id"`
		ProgramName         string  `db:"program_name"`
		ProblemID           int64   `db:"problem_id"`
		ProblemName         string  `db:"problem_name"`
	}{}
	if err := db.Row(ctx, &solution, `
		SELECT
			solution_id,
			solution_booster,
			solution_score,
			solution_modified,
			solution_description,
			solution_data_blob,
			solution_data_error,
			program_id,
			program_name,
			problem_id,
			problem_name
		FROM
			solutions
			NATURAL LEFT JOIN solution_data
			NATURAL LEFT JOIN programs
			NATURAL LEFT JOIN problems
		WHERE
			solution_id = ?
		LIMIT 1
		`, solutionID); err != nil {
		return "", err
	}
	var buf bytes.Buffer
	if err = tmpl.Execute(&buf, solution); err != nil {
		return "", err
	}
	return HTML(buf.String()), nil
}
