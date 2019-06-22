package api

import (
	"context"

	"github.com/imos/icfpc2019/go/util/db"
	"github.com/imos/icfpc2019/go/util/pb"
	"github.com/pkg/errors"
)

func insertProblemHandler(
	ctx context.Context, apiReq *pb.Api_Request, apiResp *pb.Api_Response,
) error {
	req := apiReq.GetInsertProblem()
	resp := &pb.Api_Response_InsertProblem{}
	if req == nil {
		return nil
	}

	if req.GetProblemName() == "" {
		return errors.New("problem_name is missing")
	}
	if req.GetProblemData() == nil || len(req.GetProblemData()) == 0 {
		return errors.New("problem_data is missing")
	}

	tx, err := db.DB().BeginTxx(ctx, nil)
	if err != nil {
		return errors.WithStack(err)
	}
	if err := func() error {
		res, err := tx.ExecContext(ctx,
			"INSERT problems(problem_name) VALUES(?)", req.GetProblemName())
		if err != nil {
			return err
		}
		id, err := res.LastInsertId()
		if err != nil {
			return errors.WithStack(err)
		}
		res, err = tx.ExecContext(ctx,
			"INSERT problem_data(problem_id, problem_data_blob) VALUES(?, ?)",
			id, req.GetProblemData())
		if err != nil {
			return err
		}
		_, err = tx.ExecContext(
			ctx,
			`INSERT IGNORE INTO
					solutions(program_id, problem_id, solution_lock)
					SELECT
						program_id,
						? AS problem_id,
						NOW() - INTERVAL (RAND() + 1) * 24 * 60 * 60 SECOND
							AS solution_lock
					FROM programs`,
			id)
		if err != nil {
			return err
		}
		resp.ProblemId = id
		return nil
	}(); err != nil {
		tx.Rollback()
		return err
	}
	apiResp.InsertProblem = resp
	return tx.Commit()
}
