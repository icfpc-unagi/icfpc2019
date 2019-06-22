package apiutil

import (
	"bytes"
	"context"
	"io/ioutil"
	"net/http"
	"os"
	"time"

	"github.com/pkg/errors"

	"github.com/golang/protobuf/proto"
	"github.com/imos/icfpc2019/go/util/pb"
)

func Call(ctx context.Context, req *pb.Api_Request) (*pb.Api_Response, error) {
	apiEndpoint := os.Getenv("API_ENDPOINT")
	if apiEndpoint == "" {
		apiEndpoint = "https://dashboard.sx9.jp/api/"
	}
	if reqBuf, err := proto.Marshal(req); err != nil {
		return nil, err
	} else if httpReq, err := http.NewRequest(
		http.MethodPost, apiEndpoint,
		bytes.NewReader(reqBuf)); err != nil {
		return nil, err
	} else {
		httpReq = httpReq.WithContext(ctx)
		httpReq.Header.Set("Content-Type", "application/protobuf")
		httpReq.Header.Set("Accept", "application/protobuf")
		client := http.Client{}
		if deadline, ok := ctx.Deadline(); ok {
			client.Timeout = deadline.Sub(time.Now())
		}
		httpResp, err := client.Do(httpReq)
		if err != nil {
			return nil, err
		}
		defer httpResp.Body.Close()
		resp := &pb.Api_Response{}
		if httpResp.StatusCode != 200 {
			respBuf, _ := ioutil.ReadAll(httpResp.Body)
			return nil, errors.Errorf(
				"invalid response code: %d: %s", httpResp.StatusCode, respBuf)
		} else if respBuf, err := ioutil.ReadAll(httpResp.Body); err != nil {
			return nil, errors.Errorf("failed to read response: %s", err)
		} else if ct := httpResp.Header.Get(
			"Content-Type"); ct != "application/protobuf" {
			return nil, errors.Errorf("context type is invalid: %s", ct)
		} else if err := proto.Unmarshal(respBuf, resp); err != nil {
			return nil, errors.Errorf("failed to decode response: %s", err)
		}
		return resp, nil
	}
}
