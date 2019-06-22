package api

import (
	"context"
	"database/sql"

	"github.com/imos/icfpc2019/go/util/db"
	"github.com/imos/icfpc2019/go/util/pb"
	"github.com/pkg/errors"
)

func insertSolutionHandler(
	ctx context.Context, apiReq *pb.Api_Request, apiResp *pb.Api_Response,
) error {
	req := apiReq.GetInsertSolution()
	resp := &pb.Api_Response_InsertSolution{}
	if req == nil {
		return nil
	}

	tx, err := db.DB().BeginTxx(ctx, nil)
	if err != nil {
		return errors.WithStack(err)
	}
	if err := func() error {
		if err := tx.GetContext(ctx, &struct {
			ProgramID int64 `db:"program_id"`
		}{},
			`SELECT program_id FROM programs WHERE program_id = ?`,
			req.GetProgramId()); err != nil {
			return errors.Errorf("failed to look up a program: %v", err)
		}
		totalAffected := int64(0)
		if req.GetStrategy() == pb.Api_Request_InsertSolution_REPLACE {
			if req.GetProblemId() == 0 {
				_, err = tx.ExecContext(ctx, `
				UPDATE solutions
				SET solution_lock =
					NOW() - INTERVAL (RAND() + 1) * 24 * 60 * 60 SECOND
				WHERE program_id = ?`, req.GetProgramId())
			} else {
				_, err = tx.ExecContext(ctx, `
				UPDATE solutions
				SET solution_lock =
					NOW() - INTERVAL (RAND() + 1) * 24 * 60 * 60 SECOND
				WHERE program_id = ? AND problem_id = ?`,
					req.GetProgramId(), req.GetProblemId())
			}
			if err != nil {
				return errors.WithStack(err)
			}
		}
		if req.GetStrategy() == pb.Api_Request_InsertSolution_IGNORE ||
			req.GetStrategy() == pb.Api_Request_InsertSolution_REPLACE {
			result, err := func() (sql.Result, error) {
				if req.GetProblemId() == 0 {
					return tx.ExecContext(
						ctx,
						`INSERT IGNORE INTO
							solutions(program_id, problem_id, solution_lock)
						SELECT
							? AS program_id,
							problem_id,
							NOW() - INTERVAL (RAND() + 1) * 24 * 60 * 60 SECOND
								AS solution_lock
						FROM problems`,
						req.GetProgramId())
				}
				return tx.ExecContext(
					ctx,
					`INSERT IGNORE INTO
						solutions(program_id, problem_id, solution_lock)
						VALUES (
							?, ?,
							NOW() - INTERVAL (RAND() + 1) *
								24 * 60 * 60 SECOND)`,
					req.GetProgramId(), req.GetProblemId())
			}()
			if err != nil {
				return errors.WithStack(err)
			}
			affected, err := result.RowsAffected()
			if err != nil {
				return errors.WithStack(err)
			}
			totalAffected += affected
		}
		return nil
	}(); err != nil {
		tx.Rollback()
		return err
	}
	apiResp.InsertSolution = resp
	return tx.Commit()
}
