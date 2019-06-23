package api

import (
	"context"
	"sync"

	"github.com/imos/icfpc2019/go/util/db"
	"github.com/imos/icfpc2019/go/util/pb"
	"github.com/pkg/errors"
	"google.golang.org/appengine/log"
)

var acquireSolutionLock sync.Mutex

func acquireSolutionHandler(
	ctx context.Context, apiReq *pb.Api_Request, apiResp *pb.Api_Response,
) error {
	req := apiReq.GetAcquireSolution()
	resp := &pb.Api_Response_AcquireSolution{}
	if req == nil {
		return nil
	}

	tx, err := db.DB().BeginTxx(ctx, nil)
	if err != nil {
		return errors.WithStack(err)
	}
	solutionID, err := func() (int64, error) {
		acquireSolutionLock.Lock()
		defer acquireSolutionLock.Unlock()
		acquired, err := func() (bool, error) {
			result, err := tx.ExecContext(ctx, `
				UPDATE solutions
				SET
					solution_id = (@solution_id := solution_id),
					solution_lock = NOW() + INTERVAL 5 MINUTE
				WHERE
					solution_lock < NOW()
				ORDER BY solution_lock
				LIMIT 1`)
			if err != nil {
				return false, err
			}
			affected, err := result.RowsAffected()
			if err != nil {
				return false, errors.WithStack(err)
			}
			return affected > 0, nil
		}()
		if err != nil {
			return 0, err
		}
		if !acquired {
			log.Infof(ctx, "no solution is acquired")
			return 0, nil
		}
		row := struct {
			SolutionID int64 `db:"solution_id"`
		}{}
		if err := tx.GetContext(
			ctx, &row, `SELECT @solution_id AS solution_id`); err != nil {
			return 0, errors.WithStack(err)
		}
		return row.SolutionID, nil
	}()
	if err != nil {
		tx.Rollback()
		return err
	}
	tx.Commit()

	if err := func() error {
		if solutionID == 0 {
			return nil
		}
		row := struct {
			SolutionID      int64  `db:"solution_id"`
			SolutionBooster string `db:"solution_booster"`
			ProgramID       int64  `db:"program_id"`
			ProgramName     string `db:"program_name"`
			ProgramCode     string `db:"program_code"`
			ProblemID       int64  `db:"problem_id"`
			ProblemName     string `db:"problem_name"`
			ProblemDataBlob []byte `db:"problem_data_blob"`
		}{}
		if err := db.DB().GetContext(ctx, &row, `
			SELECT 
				solution_id,
				solution_booster,
				program_id,
				program_name,
				program_code,
				problem_id,
				problem_name,
				problem_data_blob
			FROM
				solutions
				NATURAL LEFT JOIN programs
				NATURAL LEFT JOIN problems
				NATURAL LEFT JOIN problem_data
			WHERE solution_id = ?
			LIMIT 1`, solutionID); err != nil {
			return errors.WithStack(err)
		}
		resp.SolutionId = row.SolutionID
		resp.SolutionBooster = row.SolutionBooster
		resp.ProgramId = row.ProgramID
		resp.ProgramName = row.ProgramName
		resp.ProgramCode = row.ProgramCode
		resp.ProblemId = row.ProblemID
		resp.ProblemName = row.ProblemName
		resp.ProblemDataBlob = row.ProblemDataBlob
		return nil
	}(); err != nil {
		return err
	}
	apiResp.AcquireSolution = resp
	return nil
}
