package main

import (
	"flag"
	"fmt"
	"io/ioutil"
	"os"
	"os/exec"
	"path"
	"strconv"
	"strings"
	"sync"
	"time"

	"github.com/pkg/errors"
)

func main() {
	flag.Parse()
	if err := func() error {
		fmt.Fprintf(os.Stderr, "Fetching block number...\n")
		blockNumber, err := getBlockNumber()
		if err != nil {
			return errors.Wrap(err, "failed to get block number")
		}
		fmt.Fprintf(os.Stderr, "Current block number: %d\n", blockNumber)
		if _, err := os.Stat(
			fmt.Sprintf("/nfs/lock/%d", blockNumber)); err == nil {
			fmt.Fprintf(os.Stderr, "Block was already submitted.\n")
			return nil
		}
		fmt.Fprintf(os.Stderr, "Fetching task...\n")
		task, err := getMiningInfo("task")
		if err != nil {
			return errors.Wrap(err, "failed to get task")
		}
		fmt.Fprintf(os.Stderr, "Current task: %s\n", strings.TrimSpace(task))
		fmt.Fprintf(os.Stderr, "Fetching puzzle...\n")
		puzzle, err := getMiningInfo("puzzle")
		if err != nil {
			return errors.Wrap(err, "failed to get puzzle")
		}
		fmt.Fprintf(
			os.Stderr, "Current puzzle: %s\n", strings.TrimSpace(puzzle))
		fmt.Fprintf(os.Stderr, "Starting jobs...\n")

		dir, err := ioutil.TempDir("", "mining")
		if err := ioutil.WriteFile(
			path.Join(dir, "task.desc"), []byte(task), 0644); err != nil {
			return errors.Errorf("failed to prepare task.desc: %s", err)
		}
		if err := ioutil.WriteFile(
			path.Join(dir, "puzzle.cond"), []byte(puzzle), 0644); err != nil {
			return errors.Errorf("failed to prepare puzzle.cond: %s", err)
		}
		os.Setenv("TASK_FILE", path.Join(dir, "task.desc"))
		os.Setenv("PUZZLE_FILE", path.Join(dir, "puzzle.cond"))

		wg := sync.WaitGroup{}
		wg.Add(1)
		go func() {
			defer wg.Done()
			generatePuzzle()
			fmt.Fprintf(os.Stderr, "Puzzle generation finished.\n")
		}()
		wg.Add(1)
		go func() {
			defer wg.Done()
			solveTask()
			fmt.Fprintf(os.Stderr, "Task solver finished.\n")
		}()
		wg.Done()

		return nil
	}(); err != nil {
		panic(fmt.Sprintf("%+v", err))
	}
	fmt.Fprintf(os.Stderr, "Mining successfully finished.\n")
}

func generatePuzzle() {

}

func solveTask() {

}

func getBlockNumber() (int64, error) {
	stdout, stderr, err := execute(
		"lambda-client", "getblockchaininfo", "block")
	if err != nil {
		fmt.Fprintf(os.Stderr, "%s\n", stderr)
		return 0, errors.Wrap(err, "failed to execute getblockchaininfo")
	}
	bn, err := strconv.ParseInt(strings.TrimSpace(stdout), 10, 64)
	if err != nil {
		return 0, errors.Errorf("failed to parse block number: %s", err)
	}
	return bn, nil
}

func getMiningInfo(kind string) (string, error) {
	stdout, stderr, err := execute(
		"lambda-client", "getmininginfo", kind)
	if err != nil {
		fmt.Fprintf(os.Stderr, "%s\n", stderr)
		return "", errors.Wrapf(
			err, "failed to execute getmininginfo: %s", kind)
	}
	return stdout, nil
}

func execute(
	name string, args ...string,
) (stdout string, stderr string, err error) {
	dir, err := ioutil.TempDir("", "mining")
	if err != nil {
		return "", "", errors.Errorf(
			"failed to create a temporary directory: %s", err)
	}
	defer func() {
		stdoutBuf, _ := ioutil.ReadFile(path.Join(dir, "stdout"))
		stderrBuf, _ := ioutil.ReadFile(path.Join(dir, "stderr"))
		stdout = string(stdoutBuf)
		stderr = string(stderrBuf)
	}()
	cmd := exec.Command(name, args...)
	cmd.Stdout, err = os.Create(path.Join(dir, "stdout"))
	if err != nil {
		return "", "", errors.Errorf(
			"failed to create stdout file: %s", err)
	}
	cmd.Stderr, err = os.Create(path.Join(dir, "stderr"))
	if err != nil {
		return "", "", errors.Errorf(
			"failed to create stderr file: %s", err)
	}

	done := make(chan struct{}, 1)
	result := make(chan error, 2)

	go func() {
		result <- cmd.Run()
		close(done)
	}()

	go func() {
		select {
		case <-done:
		case <-time.After(time.Second * 300):
			result <- errors.Errorf("deadline exceeded: %s %v", name, args)
			cmd.Process.Kill()
		}
	}()

	err = <-result
	return
}
