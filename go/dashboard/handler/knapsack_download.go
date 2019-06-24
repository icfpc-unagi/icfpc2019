package handler

import (
	"archive/zip"
	"fmt"
	"net/http"
	"regexp"
	"strconv"
	"strings"
	"time"

	"github.com/pkg/errors"

	"google.golang.org/appengine"

	"github.com/imos/icfpc2019/go/util/db"
)

func init() {
	http.HandleFunc("/knapsack_download", knapsackDownloadHandler)
	http.HandleFunc(
		"/ytueijprkwrkaqzh/knapsack_download", knapsackDownloadHandler)
}

func knapsackDownloadHandler(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()
	solutionIDs, err := func() ([]string, error) {
		input := strings.TrimSpace(r.PostFormValue("input"))
		solutionIDs := []string{}
		for i, row := range strings.Split(input, "\n") {
			cells := strings.Split(strings.TrimSpace(row), ",")
			if len(cells) < 2 {
				return nil, errors.Errorf(
					"insufficient fields in line %d", i)
			}
			solutionID, err := strconv.ParseInt(cells[2], 10, 64)
			if err != nil {
				return nil, errors.Errorf(
					"failed to parse solution ID in line %d: %s",
					i, cells[2])
			}
			solutionIDs = append(solutionIDs, fmt.Sprintf("%d", solutionID))
		}
		return solutionIDs, nil
	}()
	if err != nil {
		http.Error(w, fmt.Sprintf("Error: %+v", err), 500)
		return
	}

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
			solution_booster,
			solution_data_blob
		FROM
			solutions
			NATURAL JOIN problems
			NATURAL JOIN programs
			NATURAL JOIN solution_data
		WHERE solution_id IN (`+strings.Join(solutionIDs, ", ")+`)
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
			SolutionBooster  string `db:"solution_booster"`
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
			return
		}
		blob := []byte(solution.SolutionDataBlob)
		n, err := writer.Write(blob)
		if err != nil {
			http.Error(w, fmt.Sprintf("failed to write blob: %+v", err), 500)
			return
		}
		if n != len(blob) {
			http.Error(w, fmt.Sprintf(
				"failed to write all bytes: %d vs %d", n, len(blob)), 500)
			return
		}

		if solution.SolutionBooster != "" {
			writer, err := zipWriter.Create(
				strings.TrimSuffix(solution.ProblemName, ".desc") + ".buy")
			if err != nil {
				http.Error(w,
					fmt.Sprintf("failed to create zip writer: %+v", err), 500)
				return
			}
			blob := []byte(solution.SolutionBooster)
			n, err := writer.Write(blob)
			if err != nil {
				http.Error(w,
					fmt.Sprintf("failed to write blob: %+v", err), 500)
				return
			}
			if n != len(blob) {
				http.Error(w, fmt.Sprintf(
					"failed to write all bytes: %d vs %d", n, len(blob)), 500)
				return
			}
		}
	}
}
