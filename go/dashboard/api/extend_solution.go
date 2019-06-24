package api

import (
	"context"

	"github.com/imos/icfpc2019/go/util/db"
	"github.com/imos/icfpc2019/go/util/pb"
	"github.com/pkg/errors"
)

func extendSolutionHandler(
	ctx context.Context, apiReq *pb.Api_Request, apiResp *pb.Api_Response,
) error {
	req := apiReq.GetExtendSolution()
	resp := &pb.Api_Response_ExtendSolution{}
	if req == nil {
		return nil
	}

	tx, err := db.DB().BeginTxx(ctx, nil)
	if err != nil {
		return errors.WithStack(err)
	}
	if err := func() error {
		result, err := tx.ExecContext(ctx, `
			UPDATE solutions
			SET solution_lock = NOW() + INTERVAL 2 MINUTE
			WHERE solution_id = ? AND NOW() < solution_lock
			LIMIT 1`, req.GetSolutionId())
		if err != nil {
			return errors.WithStack(err)
		}
		affected, err := result.RowsAffected()
		if err != nil {
			return errors.WithStack(err)
		}
		if affected == 0 {
			return errors.Errorf(
				"failed to lock solution: %d", req.GetSolutionId())
		}
		return nil
	}(); err != nil {
		tx.Rollback()
		return err
	}
	apiResp.ExtendSolution = resp
	return tx.Commit()
}
