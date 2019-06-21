package main

import (
	"context"
	"fmt"
	"os"

	"github.com/golang/protobuf/proto"

	"github.com/imos/icfpc2019/go/util/apiutil"
	"github.com/imos/icfpc2019/go/util/pb"
)

func main() {
	ctx := context.Background()
	resp, err := apiutil.Call(ctx, &pb.Api_Request{})
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to call API: %v", err)
		os.Exit(1)
	}
	fmt.Printf("%s\n", proto.MarshalTextString(resp))
}
