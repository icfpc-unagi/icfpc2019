package api

import (
	"context"

	"github.com/imos/icfpc2019/go/util/db"
	"github.com/imos/icfpc2019/go/util/pb"
	"github.com/pkg/errors"
)

func acquireSolutionExtraHandler(
	ctx context.Context, apiReq *pb.Api_Request, apiResp *pb.Api_Response,
) error {
	req := apiReq.GetAcquireSolutionExtra()
	resp := &pb.Api_Response_AcquireSolutionExtra{}
	if req == nil {
		return nil
	}

	tx, err := db.DB().BeginTxx(ctx, nil)
	if err != nil {
		return errors.WithStack(err)
	}
	if err := func() error {
		row := struct {
			SolutionID           int64  `db:"solution_id"`
			ProblemDataBlob      []byte `db:"problem_data_blob"`
			SolutionDataBlob     []byte `db:"solution_data_blob"`
			SolutionDataModified string `db:"solution_data_modified"`
		}{}
		if err := db.Row(ctx, &row, `
			SELECT 
				solution_id,
				problem_data_blob,
				solution_data_blob,
				solution_data_modified
			FROM
				solutions
				NATURAL LEFT JOIN problem_data
				NATURAL LEFT JOIN solution_data
			WHERE
				solution_data_image_is_null AND
				LENGTH(solution_data_blob) > 0
			ORDER BY RAND()
			LIMIT 1`); err != nil {
			// ok to return empty
			apiResp.AcquireSolutionExtra = resp
			return nil
		}
		resp.SolutionId = row.SolutionID
		resp.ProblemDataBlob = row.ProblemDataBlob
		resp.SolutionDataBlob = row.SolutionDataBlob
		resp.SolutionDataModified = row.SolutionDataModified
		return nil
	}(); err != nil {
		tx.Rollback()
		return err
	}
	apiResp.AcquireSolutionExtra = resp
	return tx.Commit()
}
