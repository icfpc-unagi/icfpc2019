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
	"syscall"
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
	canceled := false
	for threadID := 0; threadID*16 < int(*parallel); threadID++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for {
				queue <- struct{}{}
				if canceled {
					return
				}
				resp, err := apiutil.Call(ctx, &pb.Api_Request{
					AcquireSolution: &pb.Api_Request_AcquireSolution{},
				})
				if err != nil {
					fmt.Fprintf(os.Stderr, "failed to call API: %+v\n", err)
					time.Sleep(10 * time.Second)
					continue
				}
				solution := resp.GetAcquireSolution()
				if solution.GetSolutionId() == 0 {
					canceled = true
					fmt.Fprintf(os.Stderr, "no solution to run\n")
					return
				}
				wg.Add(1)
				go func() {
					defer func() { <-queue }()
					defer wg.Done()
					if err := runOnce(ctx, solution); err != nil {
						fmt.Fprintf(os.Stderr, "%+v\n", err)
					}
				}()
			}
		}()
		time.Sleep(time.Second)
	}
	return nil
}

func runOnce(
	ctx context.Context, solution *pb.Api_Response_AcquireSolution) error {
	done := make(chan struct{}, 1)
	defer close(done)
	go func() {
		failed := 0
		for failed < 3 {
			select {
			case <-time.After(time.Second * 20):
				fmt.Fprintf(os.Stderr,
					"extending solution: %d\n", solution.GetSolutionId())
				_, err := apiutil.Call(ctx, &pb.Api_Request{
					ExtendSolution: &pb.Api_Request_ExtendSolution{
						SolutionId: solution.GetSolutionId(),
					},
				})
				if err != nil {
					fmt.Fprintf(os.Stderr, "failed to extend lock: %+v", err)
					failed++
				} else {
					failed = 0
				}
			case <-done:
				return
			}
		}
	}()

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
	if len(result.GetSolutionDataError()) > 100000 {
		result.SolutionDataError = append(
			result.SolutionDataError[0:50000],
			result.SolutionDataError[len(
				result.GetSolutionDataError())-50000:]...)
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
	if err := ioutil.WriteFile(path.Join(dir, "buy"),
		[]byte(solution.GetSolutionBooster()), 0644); err != nil {
		return errors.WithStack(err)
	}
	stderr, err := os.Create(path.Join(dir, "stderr"))
	if err != nil {
		return errors.WithStack(err)
	}
	script := fmt.Sprintf(
		"task='%s'; task_name='%s'; buy='%s'; "+
			"buy_string='%s'; solution='%s'; %s",
		path.Join(dir, "task"),
		strings.TrimSuffix(solution.GetProblemName(), ".desc"),
		path.Join(dir, "buy"),
		solution.GetSolutionBooster(),
		path.Join(dir, "solution"),
		solution.GetProgramCode())
	cmd := exec.Command("bash", "-c", script)
	cmd.SysProcAttr = &syscall.SysProcAttr{Setpgid: true}
	cmd.Stderr = stderr
	done := make(chan struct{}, 1)
	errChan := make(chan error, 5)

	go func() {
		errChan <- cmd.Run()
		fmt.Fprintf(os.Stderr, "solver finished\n")
		close(done)
	}()

	go func() {
		select {
		case <-done:
		case <-time.After(time.Second * 2400):
			errChan <- errors.New("deadline exceeded")
			syscall.Kill(-cmd.Process.Pid, syscall.SIGKILL)
			time.Sleep(30)
			cmd.Process.Kill()
		}
	}()

	cmdErr := <-errChan
	result.SolutionDataError, _ = ioutil.ReadFile(path.Join(dir, "stderr"))
	result.SolutionDataBlob, _ = ioutil.ReadFile(path.Join(dir, "solution"))
	fmt.Fprintf(os.Stderr, "solver finalized\n")

	var output []byte
	for i := 0; i < 2; i++ {
		var timeout bool
		output, timeout, err = commandWithTimeout("/nfs/bin/solution_checker",
			path.Join(dir, "task"),
			path.Join(dir, "solution"),
			path.Join(dir, "buy"))
		if !timeout {
			break
		}
		fmt.Fprintf(os.Stderr, "scorer failed: %s: %d: %s: %+v\n",
			output, solution.GetSolutionId(), solution.GetProgramCode(), err)
	}
	result.SolutionDataError = append(
		result.GetSolutionDataError(), []byte(output)...)
	if err != nil {
		result.SolutionDataError = append(
			result.GetSolutionDataError(), []byte(err.Error())...)
	}
	if matches := regexp.MustCompile(
		`Success!\s+Your solution took (\d+) time units\.`,
	).FindStringSubmatch(string(output)); matches != nil && len(matches) > 0 {
		result.SolutionScore, err = strconv.ParseInt(matches[1], 10, 64)
		if err != nil {
			result.SolutionDataError = append(
				result.GetSolutionDataError(), []byte(err.Error())...)
		}
	}
	fmt.Fprintf(os.Stderr, "validation result: %s\n",
		strings.TrimSpace(string(output)))
	fmt.Fprintf(os.Stderr, "validator finished\n")

	return cmdErr
}

func commandWithTimeout(
	name string, args ...string,
) (output []byte, timeout bool, err error) {
	fmt.Fprintf(os.Stderr, "running %s %v\n", name, args)

	cmd := exec.Command(name, args...)
	done := make(chan struct{}, 1)
	errs := make(chan error, 20)

	go func() {
		var err error
		output, err = cmd.CombinedOutput()
		errs <- err
		close(done)
	}()

	go func() {
		select {
		case <-done:
		case <-time.After(time.Second * 120):
			timeout = true
			cmd.Process.Kill()
		}
	}()

	go func() {
		select {
		case <-done:
		case <-time.After(time.Second * 180):
			timeout = true
			errs <- errors.New("deadline exceeded")
			time.Sleep(time.Second)
			cmd.Process.Kill()
		}
	}()

	err = <-errs
	return
}
