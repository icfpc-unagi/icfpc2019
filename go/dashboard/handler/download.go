package handler

import (
	"archive/zip"
	"fmt"
	"net/http"
	"regexp"
	"strings"
	"time"

	"google.golang.org/appengine"

	"github.com/imos/icfpc2019/go/util/db"
)

func init() {
	http.HandleFunc("/download", downloadHandler)
	http.HandleFunc("/ytueijprkwrkaqzh/download", downloadHandler)
}

func downloadHandler(w http.ResponseWriter, r *http.Request) {
	ctx := appengine.NewContext(r)

	w.Header().Set("Content-Type", "application/zip")
	w.Header().Set("Content-Disposition",
		"attachment; filename=solutions-"+
			time.Now().UTC().Format("20060102-150405")+".zip")

	zipWriter := zip.NewWriter(w)
	defer zipWriter.Close()

	rows, err := db.DB().QueryxContext(ctx, `
		SELECT
			solution_id,
			program_name,
			problem_name,
			solution_data_blob
		FROM
			(SELECT
				problem_id,
				MIN(solution_id) AS solution_id
			FROM
				(SELECT
					problem_id,
					MIN(solution_score) AS solution_score
				FROM solutions
				WHERE
					solution_score IS NOT NULL AND
					solution_booster == ""
				GROUP BY problem_id) AS t
				NATURAL JOIN solutions
			GROUP BY problem_id) AS t
			NATURAL JOIN solutions
			NATURAL JOIN problems
			NATURAL JOIN programs
			NATURAL JOIN solution_data
		ORDER BY problem_name`)
	if err != nil {
		http.Error(w, fmt.Sprintf("failed to query: %+v", err), 500)
		return
	}
	for rows.Next() {
		solution := &struct {
			SolutionID       int64  `db:"solution_id"`
			ProgramName      string `db:"program_name"`
			ProblemName      string `db:"problem_name"`
			SolutionDataBlob string `db:"solution_data_blob"`
		}{}
		if err := rows.StructScan(&solution); err != nil {
			http.Error(w, fmt.Sprintf("failed to scan: %+v", err), 500)
		}
		if !regexp.MustCompile(
			`^prob-.*\.desc$`).MatchString(solution.ProblemName) {
			continue
		}
		writer, err := zipWriter.Create(
			strings.TrimSuffix(solution.ProblemName, ".desc") + ".sol")
		if err != nil {
			http.Error(w, fmt.Sprintf("failed to create zip writer: %+v", err), 500)
		}
		blob := []byte(solution.SolutionDataBlob)
		n, err := writer.Write(blob)
		if err != nil {
			http.Error(w, fmt.Sprintf("failed to write blob: %+v", err), 500)
		}
		if n != len(blob) {
			http.Error(w, fmt.Sprintf(
				"failed to write all bytes: %d vs %d", n, len(blob)), 500)
		}
	}
}
