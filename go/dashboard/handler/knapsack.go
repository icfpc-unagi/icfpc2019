package handler

import (
	"fmt"
	"net/http"
	"regexp"
	"strings"

	"google.golang.org/appengine"

	"github.com/imos/icfpc2019/go/util/db"
)

func init() {
	http.HandleFunc("/knapsack", knapsackHandler)
	http.HandleFunc("/ytueijprkwrkaqzh/knapsack", knapsackHandler)
}

func knapsackHandler(w http.ResponseWriter, r *http.Request) {
	ctx := appengine.NewContext(r)

	w.Header().Set("Content-Type", "text/plain")

	rows, err := db.DB().QueryxContext(ctx, `
		SELECT
			problem_name,
			solution_booster,
			solution_id,
			solution_score
		FROM
			(SELECT
				problem_id,
				solution_booster,
				MIN(solution_id) AS solution_id
			FROM
				(SELECT
					problem_id,
					solution_booster,
					MIN(solution_score) AS solution_score
				FROM solutions
				WHERE
					solution_score IS NOT NULL
				GROUP BY problem_id, solution_booster) AS t
				NATURAL JOIN solutions
			GROUP BY problem_id, solution_booster) AS t
			NATURAL JOIN solutions
			NATURAL JOIN problems
		ORDER BY problem_name, solution_booster`)
	if err != nil {
		http.Error(w, fmt.Sprintf("failed to query: %+v", err), 500)
		return
	}
	for rows.Next() {
		s := &struct {
			ProblemName     string `db:"problem_name"`
			SolutionBooster string `db:"solution_booster"`
			SolutionID      int64  `db:"solution_id"`
			SolutionScore   int64  `db:"solution_score"`
		}{}
		if err := rows.StructScan(&s); err != nil {
			http.Error(w, fmt.Sprintf("failed to scan: %+v", err), 500)
		}
		if !regexp.MustCompile(
			`^prob-.*\.desc$`).MatchString(s.ProblemName) {
			continue
		}
		w.Write([]byte(
			fmt.Sprintf("%s,%s,%d,%d\n",
				strings.TrimSuffix(s.ProblemName, ".desc"),
				s.SolutionBooster,
				s.SolutionID,
				s.SolutionScore)))
	}
}
