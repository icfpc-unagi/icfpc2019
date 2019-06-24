package handler

import (
	"context"
	"fmt"
	"html/template"
	"net/http"
	"strings"

	"github.com/imos/icfpc2019/go/util/metadata"

	"github.com/imos/icfpc2019/go/util/db"
)

func init() {
	registerHandler("/problems/", problemsHandler)
}

func problemsHandler(ctx context.Context, r *http.Request) (HTML, error) {
	problems := []struct {
		ProblemID       int64  `db:"problem_id"`
		ProblemName     string `db:"problem_name"`
		ProblemDataBlob string `db:"problem_data_blob"`
		ProblemCreated  string `db:"problem_created"`
	}{}
	if err := db.Select(ctx, &problems,
		`SELECT problem_id, problem_name, problem_data_blob, problem_created `+
			`FROM problems NATURAL JOIN problem_data  ORDER BY problem_name`); err != nil {
		return "", err
	}
	sb := &strings.Builder{}
	sb.WriteString(
		`<table class="table table-clickable">` +
			`<thead><tr><td>ID</td><td width="400">Image</td><td>Name</td><td>Size</td><td>Boosters</td><td>Created</td></thead>` +
			`<tbody>`)

	for _, problem := range problems {
		md, err := metadata.GetTaskMetadata(problem.ProblemDataBlob)
		if err != nil {
			return "", err
		}
		id := fmt.Sprintf("%d", problem.ProblemID)
		sb.WriteString("<tr><td>")
		sb.WriteString(id)
		sb.WriteString(`</td><td><img src="/problem_image?problem_id=`)
		sb.WriteString(id)
		sb.WriteString(`" class="w400 pix"></td><td>`)
		sb.WriteString(template.HTMLEscapeString(problem.ProblemName))
		fmt.Fprintf(sb, `</td><td>(%v,%v)</td><td><code>%v</code></td><td>`, md.MaxX, md.MaxY, md.Boosters)
		sb.WriteString(template.HTMLEscapeString(problem.ProblemCreated))
		sb.WriteString("</td></tr>")
	}
	sb.WriteString(`</tbody></table>`)
	return HTML(sb.String()), nil
}
