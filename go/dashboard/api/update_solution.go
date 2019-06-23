package api

import (
	"context"

	"github.com/imos/icfpc2019/go/util/db"
	"github.com/imos/icfpc2019/go/util/pb"
	"github.com/pkg/errors"
)

func updateSolutionHandler(
	ctx context.Context, apiReq *pb.Api_Request, apiResp *pb.Api_Response,
) error {
	req := apiReq.GetUpdateSolution()
	resp := &pb.Api_Response_UpdateSolution{}
	if req == nil {
		return nil
	}

	tx, err := db.DB().BeginTxx(ctx, nil)
	if err != nil {
		return errors.WithStack(err)
	}
	if err := func() error {
		_, err := db.Execute(ctx, `
			UPDATE solutions
			SET
				solution_score = NULLIF(?, 0),
				solution_lock = NULL
			WHERE solution_id = ?
			LIMIT 1`,
			req.GetSolutionScore(),
			req.GetSolutionId())
		if err != nil {
			return err
		}
		_, err = db.Execute(ctx, `
			REPLACE INTO solution_data(
				solution_id,
				solution_data_blob,
				solution_data_error)
			VALUES (?, ?, ?)`,
			req.GetSolutionId(),
			nonEmptyBytes(req.GetSolutionDataBlob()),
			nonEmptyBytes(req.GetSolutionDataError()))
		if err != nil {
			return err
		}
		return nil
	}(); err != nil {
		tx.Rollback()
		return err
	}
	apiResp.UpdateSolution = resp
	return tx.Commit()
}

func nonEmptyBytes(b []byte) []byte {
	if b != nil {
		return b
	}
	return []byte{}
}
