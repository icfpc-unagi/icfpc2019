package handler

import (
	"context"
	"fmt"
	"math"
	"net/http"
	"os"
	"regexp"
	"sort"

	"github.com/imos/icfpc2019/go/util/db"
	"github.com/imos/icfpc2019/go/util/metadata"
	"google.golang.org/appengine/log"
)

func init() {
	registerHandler("/ranking/", rankingHandler)
}

func rankingHandler(ctx context.Context, r *http.Request) (HTML, error) {
	log.Debugf(ctx, "starting ranking handler...")
	// problem_id, program_id => index of scores
	scoreTable := map[int64]map[int64]int{}

	log.Debugf(ctx, "fetching problems...")
	problems := []struct {
		ProblemID       int64  `db:"problem_id"`
		ProblemName     string `db:"problem_name"`
		ProblemDataBlob string `db:"problem_data_blob"`
	}{}
	if err := db.Select(ctx, &problems, `
		SELECT problem_id, problem_name, problem_data_blob FROM problems NATURAL JOIN problem_data
		ORDER BY problem_name`); err != nil {
		return "", err
	}
	problemNameByID := map[int64]string{}
	problemSizeByID := map[int64]int64{}
	for _, problem := range problems {
		problemNameByID[problem.ProblemID] = problem.ProblemName
		md, err := metadata.GetTaskMetadata(problem.ProblemDataBlob)
		if err != nil {
			return "", err
		}
		fmt.Fprintf(os.Stderr, "%v", md)
		problemSizeByID[problem.ProblemID] = md.MaxX * md.MaxY
		scoreTable[problem.ProblemID] = map[int64]int{}
	}

	log.Debugf(ctx, "fetching programs...")
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

	log.Debugf(ctx, "fetching solutions...")
	type Score struct {
		ProblemID     int64 `db:"problem_id"`
		ProgramID     int64 `db:"program_id"`
		SolutionID    int64 `db:"solution_id"`
		SolutionScore int64 `db:"solution_score"`
		ComputedScore int64
	}
	scores := []Score{}
	if err := db.Select(ctx, &scores, `
		SELECT
			program_id,
			problem_id,
			MAX(solution_id) AS solution_id,
			MIN(solution_score) AS solution_score
		FROM solutions NATURAL JOIN problem_data
		WHERE
			solution_score IS NOT NULL AND
			solution_booster = ""
		GROUP BY program_id, problem_id`); err != nil {
		return "", err
	}

	log.Debugf(ctx, "calculating best scores...")
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

	log.Debugf(ctx, "calculating total scores...")
	// program_id => sum(ComputedScore)
	totalScores := map[int64]int64{}
	for idx, score := range scores {
		bestScore := scores[bestScores[score.ProblemID]].SolutionScore
		myScore := score.SolutionScore
		size := problemSizeByID[score.ProblemID]
		computedScore := int64(
			math.Ceil(1000 * math.Log2(float64(size)) * float64(bestScore) / float64(myScore)))
		if myScore >= 100000000 {
			computedScore = 0
		}
		scores[idx].ComputedScore = computedScore
		totalScores[score.ProgramID] += computedScore
	}

	log.Debugf(ctx, "listing program IDs...")
	programIDs := []int64{}
	for programID := range totalScores {
		programIDs = append(programIDs, programID)
	}
	sort.SliceStable(programIDs, func(i, j int) bool {
		return totalScores[programIDs[i]] > totalScores[programIDs[j]]
	})
	seenProgramName := map[string]bool{}
	oldProgramIDs := programIDs
	programIDs = []int64{}
	for _, programID := range oldProgramIDs {
		programName := programNameByID[programID]
		programName =
			regexp.MustCompile("@.*$").ReplaceAllString(programName, "")
		if seenProgramName[programName] {
			continue
		}
		seenProgramName[programName] = true
		programIDs = append(programIDs, programID)
	}

	log.Debugf(ctx, "rendering rankings...")
	var output HTMLBuffer
	// var output HTML
	output.WriteHTML(
		`<table class="table table-clickable">`,
		`<thead><tr><td>Problem</td><td colspan="2" align="center">Best</td>`)
	for i, programID := range programIDs {
		if i > 30 {
			break
		}
		output.WriteHTML(`<td colspan="2" align="center">`)
		output.WriteString(fmt.Sprintf("%d-th", i))
		output.WriteHTML(`<br><a href="/program?program_id=`)
		output.WriteString(fmt.Sprintf("%d", programID))
		output.WriteHTML(`">`)
		output.WriteString(programNameByID[programID])
		output.WriteHTML("</a></td>")
	}
	output.WriteHTML(`</thead><tbody>`)
	appendScore := func(s *Score, best bool) {
		note := Escape(fmt.Sprintf("%d", s.ComputedScore))
		if best {
			note = `<a href="/program?program_id=` +
				Escape(fmt.Sprintf("%d", s.ProgramID)) + `">` +
				Escape(programNameByID[s.ProgramID]) + `</a>`
		}
		output.WriteHTML(
			`<td align="right"><a href="/solution?solution_id=`)
		output.WriteString(fmt.Sprintf("%d", s.SolutionID))
		output.WriteHTML(`">`)
		if s.SolutionScore >= 100000000 {
			output.WriteHTML(
				`invalid</a></td><td>(`, note, ")</td>")
			return
		}
		output.WriteString(fmt.Sprintf("%d", s.SolutionScore))
		output.WriteHTML("</a></td><td>(", note, ")</td>")
	}
	for _, problem := range problems {
		output.WriteHTML("<tr><td>")
		output.WriteString(problem.ProblemName)
		output.WriteHTML("</td>")

		appendScore(&scores[bestScores[problem.ProblemID]], true)

		programIDToScore := scoreTable[problem.ProblemID]
		for i, programID := range programIDs {
			if i > 30 {
				break
			}
			appendScore(&scores[programIDToScore[programID]], false)
		}
		output.WriteHTML("</tr>")
	}
	output.WriteHTML(`</tbody></table>`)
	log.Debugf(ctx, "finished rendering")
	return output.HTML(), nil
}
