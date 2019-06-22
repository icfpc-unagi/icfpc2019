package api

import (
	"context"

	"github.com/imos/icfpc2019/go/util/db"
	"github.com/imos/icfpc2019/go/util/pb"
	"github.com/pkg/errors"
)

func updateSolutionExtraHandler(
	ctx context.Context, apiReq *pb.Api_Request, apiResp *pb.Api_Response,
) error {
	req := apiReq.GetUpdateSolutionExtra()
	resp := &pb.Api_Response_UpdateSolutionExtra{}
	if req == nil {
		return nil
	}

	tx, err := db.DB().BeginTxx(ctx, nil)
	if err != nil {
		return errors.WithStack(err)
	}
	if err := func() error {
		_, err := tx.ExecContext(
			ctx,
			`UPDATE solution_data
			SET solution_data_image = ?
			WHERE solution_id = ? AND solution_data_modified = ?`,
			req.GetSolutionDataImage(),
			req.GetSolutionId(),
			req.GetSolutionDataModified())
		if err != nil {
			return err
		}
		return nil
	}(); err != nil {
		tx.Rollback()
		return err
	}
	apiResp.UpdateSolutionExtra = resp
	return tx.Commit()
}
