package api

import (
	"context"

	"github.com/imos/icfpc2019/go/util/db"
	"github.com/imos/icfpc2019/go/util/pb"
	"github.com/pkg/errors"
)

func updateProblemExtraHandler(
	ctx context.Context, apiReq *pb.Api_Request, apiResp *pb.Api_Response,
) error {
	req := apiReq.GetUpdateProblemExtra()
	resp := &pb.Api_Response_UpdateProblemExtra{}
	if req == nil {
		return nil
	}

	if req.GetProblemDataImage() == nil || len(req.GetProblemDataImage()) == 0 {
		return errors.New("problem_data_image is missing")
	}

	tx, err := db.DB().BeginTxx(ctx, nil)
	if err != nil {
		return errors.WithStack(err)
	}
	if err := func() error {
		image := req.GetProblemDataImage()
		if image == nil {
			image = []byte{}
		}
		_, err := tx.ExecContext(ctx,
			`UPDATE problem_data
			SET problem_data_image = ?
			WHERE problem_id = ?`,
			image, req.GetProblemId())
		if err != nil {
			return err
		}
		return nil
	}(); err != nil {
		tx.Rollback()
		return err
	}
	apiResp.UpdateProblemExtra = resp
	return tx.Commit()
}
