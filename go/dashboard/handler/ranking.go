package handler

import (
	"context"
	"fmt"
	"math"
	"net/http"
	"sort"

	"github.com/imos/icfpc2019/go/util/db"
)

func init() {
	registerHandler("/ranking/", rankingHandler)
}

func rankingHandler(ctx context.Context, r *http.Request) (HTML, error) {
	// problem_id, program_id => index of scores
	scoreTable := map[int64]map[int64]int{}

	problems := []struct {
		ProblemID   int64  `db:"problem_id"`
		ProblemName string `db:"problem_name"`
	}{}
	if err := db.Select(ctx, &problems, `
		SELECT problem_id, problem_name FROM problems
		ORDER BY problem_name`); err != nil {
		return "", err
	}
	problemNameByID := map[int64]string{}
	for _, problem := range problems {
		problemNameByID[problem.ProblemID] = problem.ProblemName
		scoreTable[problem.ProblemID] = map[int64]int{}
	}

	programs := []struct {
		ProgramID   int64  `db:"program_id"`
		ProgramName string `db:"program_name"`
	}{}
	if err := db.Select(ctx, &programs, `
		SELECT program_id, program_name FROM programs`); err != nil {
		return "", err
	}
	programNameByID := map[int64]string{}
	for _, program := range programs {
		programNameByID[program.ProgramID] = program.ProgramName
	}

	type Score struct {
		ProblemID     int64 `db:"problem_id"`
		ProgramID     int64 `db:"program_id"`
		SolutionScore int64 `db:"solution_score"`
		ComputedScore int64
	}
	scores := []Score{}
	if err := db.Select(ctx, &scores, `
		SELECT
			program_id,
			problem_id,
			MIN(solution_score) AS solution_score
		FROM solutions
		WHERE solution_score IS NOT NULL
		GROUP BY program_id, problem_id`); err != nil {
		return "", err
	}
	// problem_id => index of scores for best score
	bestScores := map[int64]int{}
	for idx, score := range scores {
		scoreTable[score.ProblemID][score.ProgramID] = idx
		if bestIdx, ok := bestScores[score.ProblemID]; !ok {
			bestScores[score.ProblemID] = idx
		} else if scores[bestIdx].SolutionScore > score.SolutionScore {
			bestScores[score.ProblemID] = idx
		}
	}
	// program_id => sum(ComputedScore)
	totalScores := map[int64]int64{}
	for idx, score := range scores {
		bestScore := scores[bestScores[score.ProblemID]].SolutionScore
		myScore := score.SolutionScore
		computedScore := int64(
			math.Ceil(1000 * float64(bestScore) / float64(myScore)))
		if myScore >= 100000000 {
			computedScore = 0
		}
		scores[idx].ComputedScore = computedScore
		totalScores[score.ProgramID] += computedScore
	}

	programIDs := []int64{}
	for programID := range totalScores {
		programIDs = append(programIDs, programID)
	}
	sort.SliceStable(programIDs, func(i, j int) bool {
		return totalScores[programIDs[i]] > totalScores[programIDs[j]]
	})

	var output HTML
	output = `<table class="table table-clickable">` +
		`<thead><tr><td>Problem</td><td colspan="2" align="center">Best</td>`
	for i, programID := range programIDs {
		if i > 10 {
			break
		}
		output += `<td colspan="2" align="center">` +
			Escape(fmt.Sprintf("%d-th", i)) +
			`<br><a href="/program?program_id=` + Escape(fmt.Sprintf("%d", programID)) + `">` +
			Escape(programNameByID[programID]) +
			"</a></td>"
	}
	output += `</thead><tbody>`
	renderScore := func(s *Score, best bool) HTML {
		note := Escape(fmt.Sprintf("%d", s.ComputedScore))
		if best {
			note = `<a href="/program?program_id=` + Escape(fmt.Sprintf("%d", s.ProgramID)) + `">` + Escape(programNameByID[s.ProgramID]) + `</a>`
		}
		if s.SolutionScore >= 100000000 {
			return `<td align="right">invalid</td><td>(` + note + ")</td>"
		}
		return `<td align="right">` +
			Escape(fmt.Sprintf("%d", s.SolutionScore)) +
			"</td><td>(" + note + ")</td>"
	}
	for _, problem := range problems {
		output += "<tr><td>" + Escape(problem.ProblemName) + "</td>"

		output += renderScore(&scores[bestScores[problem.ProblemID]], true)

		programIDToScore := scoreTable[problem.ProblemID]
		for i, programID := range programIDs {
			if i > 10 {
				break
			}
			output += renderScore(&scores[programIDToScore[programID]], false)
		}
		output += "</tr>"
	}
	output += `</tbody></table>`
	return output, nil
}
