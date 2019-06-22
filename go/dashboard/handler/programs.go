package handler

import (
	"context"
	"net/http"

	"github.com/imos/icfpc2019/go/util/db"
)

func init() {
	registerHandler("/programs/", programsHandler)
}

func programsHandler(ctx context.Context, r *http.Request) (HTML, error) {
	programs := []struct {
		ProgramID      int64  `db:"program_id"`
		ProgramName    string `db:"program_name"`
		ProgramCode    string `db:"program_code"`
		ProgramCreated string `db:"program_created"`
	}{}
	if err := db.Select(ctx, &programs,
		`SELECT program_id, program_name, program_code, program_created `+
			`FROM programs ORDER BY program_created`); err != nil {
		return "", err
	}
	output := HTML(
		`<table class="table table-clickable">` +
			`<thead><tr><td>Name</td><td>Code</td><td>Created</td></thead>` +
			`<tbody>`)
	for _, program := range programs {
		output += "<tr><td>" +
			Escape(program.ProgramName) +
			"</td><td><code>" +
			Escape(program.ProgramCode) +
			"</code></td><td>" +
			Escape(program.ProgramCreated) +
			"</td></tr>"
	}
	output += `</tbody></table>`
	return output, nil
}
