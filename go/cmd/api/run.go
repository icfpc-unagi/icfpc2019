package main

import (
	"context"
	"flag"
	"fmt"
	"io/ioutil"
	"os"
	"os/exec"
	"path"
	"regexp"
	"strconv"
	"strings"
	"sync"
	"time"

	"github.com/imos/icfpc2019/go/util/apiutil"
	"github.com/imos/icfpc2019/go/util/pb"
	"github.com/pkg/errors"
)

var parallel = flag.Int64("parallel", 1, "# of threads")

func run(args ...string) error {
	wg := sync.WaitGroup{}
	defer wg.Wait()

	queue := make(chan struct{}, *parallel)

	ctx := context.Background()
	for {
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
		wg.Add(1)
		queue <- struct{}{}
		go func() {
			defer func() { <-queue }()
			defer wg.Done()
			if err := runOnce(solution); err != nil {
				fmt.Fprintf(os.Stderr, "%+v\n", err)
			}
		}()
	}
}

func runOnce(solution *pb.Api_Response_AcquireSolution) error {
	ctx := context.Background()
	fmt.Fprintf(os.Stderr, "starting solution: %d\n", solution.GetSolutionId())
	result := &pb.Api_Request_UpdateSolution{
		SolutionId: solution.GetSolutionId(),
	}
	if err := runCommand(solution, result); err != nil {
		result.SolutionDataError = append(
			result.GetSolutionDataError(), []byte(fmt.Sprintf("%+v", err))...)
	}
	if result.SolutionScore == 0 {
		result.SolutionScore = 100000000
	}
	if _, err := apiutil.Call(ctx, &pb.Api_Request{
		UpdateSolution: result,
	}); err != nil {
		return err
	}
	fmt.Fprintf(os.Stderr, "finished solution: %d\n", solution.GetSolutionId())
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
	done := make(chan struct{}, 1)
	var cmdErr error

	wg := sync.WaitGroup{}
	wg.Add(1)
	go func() {
		defer wg.Done()
		cmdErr = cmd.Run()
		fmt.Fprintf(os.Stderr, "solver finished\n")
		close(done)
	}()

	wg.Add(1)
	go func() {
		defer wg.Done()
		select {
		case <-done:
		case <-time.After(time.Second * 10):
			cmd.Process.Kill()
		}
	}()
	wg.Wait()

	result.SolutionDataError, _ = ioutil.ReadFile(path.Join(dir, "stderr"))
	result.SolutionDataBlob, _ = ioutil.ReadFile(path.Join(dir, "solution"))
	fmt.Fprintf(os.Stderr, "solver finalized\n")

	var output []byte
	for i := 0; i < 3; i++ {
		var timeout bool
		output, timeout, err = commandWithTimeout("/nfs/programs/scorer",
			path.Join(dir, "task"),
			path.Join(dir, "solution"))
		if !timeout {
			break
		}
	}
	result.SolutionDataBlob = append(
		result.GetSolutionDataBlob(), []byte(output)...)
	if err != nil {
		result.SolutionDataBlob = append(
			result.GetSolutionDataBlob(), []byte(err.Error())...)
	} else if matches := regexp.MustCompile(
		`Success!\s+Your solution took (\d+) time units\.`,
	).FindStringSubmatch(string(output)); matches != nil && len(matches) > 0 {
		result.SolutionScore, err = strconv.ParseInt(matches[1], 10, 64)
		if err != nil {
			result.SolutionDataBlob = append(
				result.GetSolutionDataBlob(), []byte(err.Error())...)
		}
	}
	fmt.Fprintf(os.Stderr, "validation result: %s\n",
		strings.TrimSpace(string(output)))
	fmt.Fprintf(os.Stderr, "validator finished\n")

	return cmdErr
}

func commandWithTimeout(
	name string, arg ...string,
) (output []byte, timeout bool, err error) {
	cmd := exec.Command(name, arg...)
	done := make(chan struct{}, 1)
	wg := sync.WaitGroup{}

	wg.Add(1)
	go func() {
		defer wg.Done()
		output, err = cmd.CombinedOutput()
		close(done)
	}()

	wg.Add(1)
	go func() {
		defer wg.Done()
		select {
		case <-done:
		case <-time.After(time.Second * 180):
			err = errors.New("deadline exceeded")
			timeout = true
			time.Sleep(time.Second)
			cmd.Process.Kill()
		}
	}()

	wg.Wait()
	return
}
