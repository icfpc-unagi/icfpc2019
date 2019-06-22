package main

import (
	"context"
	"fmt"
	"io/ioutil"
	"os"
	"os/exec"
	"path"

	"github.com/imos/icfpc2019/go/util/apiutil"
	"github.com/imos/icfpc2019/go/util/pb"
	"github.com/pkg/errors"
)

func run(args ...string) error {
	ctx := context.Background()
	resp, err := apiutil.Call(ctx, &pb.Api_Request{
		AcquireSolution: &pb.Api_Request_AcquireSolution{},
	})
	if err != nil {
		return err
	}
	solution := resp.GetAcquireSolution()
	if solution.GetSolutionId() == 0 {
		fmt.Fprintf(os.Stderr, "no solution to run")
		return nil
	}
	fmt.Fprintf(os.Stderr, "starting solution: %d", solution.GetSolutionId())
	result := &pb.Api_Request_UpdateSolution{
		SolutionId: solution.GetSolutionId(),
	}
	if err := runCommand(solution, result); err != nil {
		result.SolutionDataError = append(
			result.GetSolutionDataError(), []byte(fmt.Sprintf("%+v", err))...)
	}
	if _, err := apiutil.Call(ctx, &pb.Api_Request{
		UpdateSolution: result,
	}); err != nil {
		return err
	}
	fmt.Fprintf(os.Stderr, "finished solution: %d", solution.GetSolutionId())
	return nil
}

func runCommand(
	solution *pb.Api_Response_AcquireSolution,
	result *pb.Api_Request_UpdateSolution) error {
	dir, err := ioutil.TempDir("", "run")
	if err != nil {
		return errors.WithStack(err)
	}
	if err := ioutil.WriteFile(path.Join(dir, "task"),
		solution.GetProblemDataBlob(), 0644); err != nil {
		return errors.WithStack(err)
	}
	stderr, err := os.Create(path.Join(dir, "stderr"))
	if err != nil {
		return errors.WithStack(err)
	}
	script := fmt.Sprintf(
		"task='%s'; solution='%s'; %s",
		path.Join(dir, "task"),
		path.Join(dir, "solution"),
		solution.GetProgramCode())
	cmd := exec.Command("bash", "-c", script)
	cmd.Stderr = stderr
	cmdErr := cmd.Run()
	result.SolutionDataError, _ = ioutil.ReadFile(path.Join(dir, "stderr"))
	result.SolutionDataBlob, _ = ioutil.ReadFile(path.Join(dir, "solution"))
	return cmdErr
}
