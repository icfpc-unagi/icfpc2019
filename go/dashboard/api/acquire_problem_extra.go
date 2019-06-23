package api

import (
	"context"

	"github.com/imos/icfpc2019/go/util/db"
	"github.com/imos/icfpc2019/go/util/pb"
	"github.com/pkg/errors"
)

func acquireProblemExtraHandler(
	ctx context.Context, apiReq *pb.Api_Request, apiResp *pb.Api_Response,
) error {
	req := apiReq.GetAcquireProblemExtra()
	resp := &pb.Api_Response_AcquireProblemExtra{}
	if req == nil {
		return nil
	}

	tx, err := db.DB().BeginTxx(ctx, nil)
	if err != nil {
		return errors.WithStack(err)
	}
	if err := func() error {
		row := struct {
			ProblemID       int64  `db:"problem_id"`
			ProblemDataBlob []byte `db:"problem_data_blob"`
		}{}
		if err := db.Row(ctx, &row, `
			SELECT
				problem_id,
				problem_data_blob
			FROM
				problem_data
			WHERE
				problem_data_image_is_null
			ORDER BY RAND()
			LIMIT 1`); err != nil {
			// ok to return empty
			apiResp.AcquireProblemExtra = resp
			return nil
		}
		resp.ProblemId = row.ProblemID
		resp.ProblemDataBlob = row.ProblemDataBlob
		return nil
	}(); err != nil {
		tx.Rollback()
		return err
	}
	apiResp.AcquireProblemExtra = resp
	return tx.Commit()
}
