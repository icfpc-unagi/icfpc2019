package main

import (
	"context"
	"flag"
	"fmt"
	"io/ioutil"
	"path"

	"github.com/golang/protobuf/proto"
	"github.com/pkg/errors"

	"github.com/imos/icfpc2019/go/util/apiutil"
	"github.com/imos/icfpc2019/go/util/pb"
)

func main() {
	flag.Parse()
	if err := func() error {
		switch flag.Arg(0) {
		case "insert-problem":
			return insertProblem()
		case "run":
			return run(flag.Args()[1:]...)
		}
		return errors.Errorf("no such command: %s", flag.Arg(0))
	}(); err != nil {
		panic(fmt.Sprintf("%+v", err))
	}
}

func insertProblem() error {
	ctx := context.Background()
	args := flag.Args()[1:]
	for _, arg := range args {
		data, err := ioutil.ReadFile(arg)
		if err != nil {
			return errors.Errorf("failed to read file: %s: %s", arg, err)
		}
		resp, err := apiutil.Call(ctx, &pb.Api_Request{
			InsertProblem: &pb.Api_Request_InsertProblem{
				ProblemName: path.Base(arg),
				ProblemData: data,
			},
		})
		if err != nil {
			panic(fmt.Sprintf("failed to call API: %+v", err))
		}
		fmt.Printf("%s\n", proto.MarshalTextString(resp))
	}
	return nil
}
