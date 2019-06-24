package handler

import (
	"context"
	"fmt"
	"html/template"
	"net/http"
	"net/url"
	"strconv"
	"strings"

	"github.com/imos/icfpc2019/go/util/db"
	"google.golang.org/appengine"
	"google.golang.org/appengine/log"
)

func init() {
	registerHandler("/solution", solutionHandler)

	http.HandleFunc("/solution/retry", func(w http.ResponseWriter, r *http.Request) {
		ctx := appengine.NewContext(r)

		w.Header().Add("Cache-Control", "no-store")
		var err error
		if r.Method != http.MethodPost {
			http.Redirect(w, r, "/solution", http.StatusSeeOther)
		} else if solutionID, err := strconv.ParseInt(r.PostFormValue("solution_id"), 10, 64); err != nil {
			log.Errorf(ctx, "%+v", err)
			http.Error(w, fmt.Sprintf("%+v", err), 500)
		} else if ref, err := url.Parse(r.Referer()); err != nil {
			log.Errorf(ctx, "%+v", err)
			http.Error(w, fmt.Sprintf("%+v", err), 500)
		} else if ref.Path != "/solution/retry" {
			err = confirmRetry(ctx, w, r, solutionID)
		} else {
			err = triggerRetry(ctx, w, r, solutionID)
		}
		if err != nil {
			log.Errorf(ctx, "%+v", err)
			http.Error(w, fmt.Sprintf("%+v", err), 500)
		}
	})
}

var tmpl = template.Must(template.New("solution").Parse(`
{{if .SolutionRunning}}
<h2 style="background-color:aqua;border-radius:5px;text-align:center">RUNNING</h2>
{{else if and (not .SolutionDone) .SolutionScore}}
<h2 style="background-color:red;border-radius:5px;text-align:center">RETRY REQUESTED</h2>
{{end}}
	<table class="table" style="width:500px;margin:auto" align="center">
		<thead><tr><td>ID</td><td>Program</td><td>Problem</td><td>Booster</td><td>Score</td><td>Modified</td></tr></thead>
		<tbody>
			<tr>
				<td>{{.SolutionID}}</td>
				<td>{{.ProgramName}} ({{.ProgramID}})</td>
				<td>{{.ProblemName}} ({{.ProblemID}})</td>
				<td>{{.SolutionBooster}}</td>
				{{if .SolutionInvalid}}<td style="color:red">invalid<a href="#retry">?</a>{{else}}<td>{{.SolutionScore}}{{end}}</td>
				<td>{{.SolutionModified}}</td>
			</tr>
		</tbody>
	</table>

	<p>{{.SolutionDescription}}</p>

	<div style="text-align:center"><img src="/solution_image?solution_id={{.SolutionID}}" class="pix" style="min-width:100px;min-height:100px;width:auto;height:auto"></div>

	<details><summary><h3 style="display:inline-block"><a name="output">Output:</a></h3></summary>
	<pre>{{.SolutionDataBlob}}</pre></details>

	<h3><a name="error">Error:</a></h3>
	<pre>{{.SolutionDataError}}</pre>

{{if .SolutionDone}}
	<form method="POST" action="/solution/retry">
	<input type="hidden" name="solution_id" value="{{.SolutionID}}">
	<a name="retry"><input type="submit" value="Retry?"></a>
	</form>
{{end}}
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
		SolutionDone        bool    `db:"solution_done"`
		SolutionRunning     bool    `db:"solution_running"`
		SolutionModified    *string `db:"solution_modified"`
		SolutionDescription string  `db:"solution_description"`
		SolutionDataBlob    string  `db:"solution_data_blob"`
		SolutionDataError   string  `db:"solution_data_error"`
		ProgramID           int64   `db:"program_id"`
		ProgramName         string  `db:"program_name"`
		ProblemID           int64   `db:"problem_id"`
		ProblemName         string  `db:"problem_name"`
		SolutionInvalid     bool
	}{}
	if err := db.Row(ctx, &solution, `
		SELECT
			solution_id,
			solution_booster,
			solution_score,
			solution_lock IS NULL AS solution_done,
			IFNULL(solution_lock > NOW(), false) AS solution_running,
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
	if solution.SolutionScore != nil && *solution.SolutionScore >= 100000000 {
		solution.SolutionInvalid = true
	}
	var buf strings.Builder
	if err = tmpl.Execute(&buf, solution); err != nil {
		return "", err
	}
	return HTML(buf.String()), nil
}

var tmpl2 = template.Must(template.New("solution").Parse(`
<!doctype html>
<html><head></head>
<body>
<form method="POST" action="/solution/retry">
<input type="hidden" name="solution_id" value="{{.}}">
<input type="submit" value="CONFIRM RETRY">
</form>
</body>
</html>
`))

func confirmRetry(ctx context.Context, w http.ResponseWriter, r *http.Request, solutionID int64) error {
	return tmpl2.Execute(w, solutionID)
}

func triggerRetry(ctx context.Context, w http.ResponseWriter, r *http.Request, solutionID int64) error {
	if _, err := db.Execute(ctx, `
		UPDATE
			solutions
		SET
			solution_lock = NOW() - INTERVAL 1 WEEK
		WHERE
			solution_id = ?
		`, solutionID); err != nil {
		return err
	}
	http.Redirect(w, r, fmt.Sprintf("/solution?solution_id=%d", solutionID), http.StatusSeeOther)
	return nil
}
