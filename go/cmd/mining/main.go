package main

import (
	"flag"
	"fmt"
	"io/ioutil"
	"math"
	"os"
	"os/exec"
	"path"
	"regexp"
	"sort"
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

		puzzleOutput := ""
		taskOutput := ""

		wg := sync.WaitGroup{}
		wg.Add(1)
		go func() {
			defer wg.Done()
			puzzleOutput = generatePuzzle()
			fmt.Fprintf(os.Stderr, "Puzzle solver finished.\n")
		}()
		wg.Add(1)
		go func() {
			defer wg.Done()
			taskOutput = solveTask()
			fmt.Fprintf(os.Stderr, "Task solver finished.\n")
		}()
		wg.Wait()

		fmt.Fprintf(os.Stderr, "Puzzle output: %s\n", puzzleOutput)
		fmt.Fprintf(os.Stderr, "Task output: %s\n", taskOutput)

		submitDir := fmt.Sprintf(
			"/nfs/dropbox/mining/submit/%d/%d", blockNumber,
			time.Now().UnixNano())
		if err := os.MkdirAll(submitDir, 0755); err != nil {
			return errors.Errorf("failed to create submit directory: %s", err)
		}
		if err := ioutil.WriteFile(
			path.Join(submitDir, "puzzle.desc"),
			[]byte(puzzleOutput), 0644); err != nil {
			return errors.Errorf("failed to write puzzle output: %s", err)
		}
		if err := ioutil.WriteFile(
			path.Join(submitDir, "task.sol"),
			[]byte(puzzleOutput), 0644); err != nil {
			return errors.Errorf("failed to write task output: %s", err)
		}
		command := fmt.Sprintf("lambda-client submit %d %s %s",
			blockNumber,
			path.Join(submitDir, "task.sol"),
			path.Join(submitDir, "puzzle.desc"))
		if err := ioutil.WriteFile(
			path.Join(submitDir, "command.sh"),
			[]byte(command), 0644); err != nil {
			return errors.Errorf("failed to write command: %s", err)
		}
		stdout, stderr, err := execute("echo", command)
		fmt.Fprintf(os.Stderr, "%s\n", strings.TrimSpace(stdout))
		fmt.Fprintf(os.Stderr, "%s\n", strings.TrimSpace(stderr))
		return err
	}(); err != nil {
		panic(fmt.Sprintf("%+v", err))
	}
	fmt.Fprintf(os.Stderr, "Mining successfully finished.\n")
}

func generatePuzzle() string {
	type Program struct {
		Command string
		Output  string
		Score   int64
	}

	programs := []*Program{
		&Program{
			Command: "/nfs/programs/puzzle-001 ${PUZZLE_FILE} /dev/stdout",
		},
	}

	fmt.Fprintf(os.Stderr, "Solving puzzles.\n")
	wg := sync.WaitGroup{}
	for _, program := range programs {
		program := program
		wg.Add(1)
		go func() {
			defer wg.Done()
			stdout, stderr, err := execute("bash", "-c", program.Command)
			if err != nil {
				fmt.Fprintf(os.Stderr, "%s\n", strings.TrimSpace(stderr))
				fmt.Fprintf(
					os.Stderr, "Command failed: %s: %s\n", program.Command, err)
				return
			}
			program.Output = stdout
		}()
	}
	wg.Wait()

	fmt.Fprintf(os.Stderr, "Validating puzzles.\n")
	// TODO(imos): Add validation.
	for _, program := range programs {
		if program.Output != "" {
			program.Score = 1
		}
	}

	sort.SliceStable(programs, func(i, j int) bool {
		is := programs[i].Score
		js := programs[j].Score
		if is <= 0 {
			is = math.MaxInt64
		}
		if js <= 0 {
			js = math.MaxInt64
		}
		return is < js
	})

	if programs[0].Score <= 0 {
		panic(fmt.Sprintf("no puzzle generated"))
	}
	return programs[0].Output
}

func solveTask() string {
	type Program struct {
		Command string
		Output  string
		Score   int64
	}

	programs := []*Program{
		&Program{
			Command: "/nfs/programs/wata-007 ${TASK_FILE}",
		},
		&Program{
			Command: "/nfs/programs/chokudai-007 ${TASK_FILE}",
		},
		&Program{
			Command: "/nfs/programs/wata-ksplit2-c8 ${TASK_FILE}",
		},
		&Program{
			Command: "/nfs/programs/chokudai-009 ${TASK_FILE}",
		},
		&Program{
			Command: "/nfs/programs/wata-k-split2 ${TASK_FILE}",
		},
	}

	fmt.Fprintf(os.Stderr, "Solving tasks.\n")
	wg := sync.WaitGroup{}
	for _, program := range programs {
		program := program
		wg.Add(1)
		go func() {
			defer wg.Done()
			stdout, stderr, err := execute("bash", "-c", program.Command)
			if err != nil {
				fmt.Fprintf(os.Stderr, "%s\n", strings.TrimSpace(stderr))
				fmt.Fprintf(
					os.Stderr, "Command failed: %s: %s\n", program.Command, err)
				return
			}
			program.Output = stdout
		}()
	}
	wg.Wait()

	fmt.Fprintf(os.Stderr, "Validating tasks.\n")
	// TODO(imos): Add validation.
	for _, program := range programs {
		program := program
		if program.Output == "" {
			continue
		}
		wg.Add(1)
		go func() {
			defer wg.Done()
			dir, err := ioutil.TempDir("", "validator")
			if err != nil {
				panic(err)
			}
			solution := path.Join(dir, "output.sol")
			ioutil.WriteFile(solution, []byte(program.Output), 0644)
			stdout, stderr, err := execute(
				"bash", "-c", "/nfs/programs/scorer ${TASK_FILE} "+solution)
			if err != nil {
				fmt.Fprintf(os.Stderr, "%s\n", strings.TrimSpace(stderr))
				fmt.Fprintf(
					os.Stderr, "Scorerer failed: %s: %s\n",
					program.Command, err)
				return
			}
			if matches := regexp.MustCompile(
				`Success!\s+Your solution took (\d+) time units`,
			).FindStringSubmatch(stdout); matches != nil && len(matches) > 0 {
				score, err := strconv.ParseInt(matches[1], 10, 64)
				if err != nil {
					fmt.Fprintf(os.Stderr, "failed to decode score: %s", err)
					return
				}
				program.Score = score
			} else {
				fmt.Fprintf(os.Stderr, "%s\n", strings.TrimSpace(stdout))
				fmt.Fprintf(os.Stderr, "%s\n", strings.TrimSpace(stderr))
				fmt.Fprintf(
					os.Stderr, "Validation failed for %s", program.Command)
			}
		}()
	}
	wg.Wait()

	sort.SliceStable(programs, func(i, j int) bool {
		is := programs[i].Score
		js := programs[j].Score
		if is <= 0 {
			is = math.MaxInt64
		}
		if js <= 0 {
			js = math.MaxInt64
		}
		return is < js
	})

	if programs[0].Score <= 0 {
		panic(fmt.Sprintf("task is not solved"))
	}
	for idx, program := range programs {
		fmt.Fprintf(os.Stderr, "#%d score=%d (%s)\n",
			idx, program.Score, program.Command)
	}
	return programs[0].Output
}

func getBlockNumber() (int64, error) {
	stdout, stderr, err := execute(
		"/nfs/bin/lambda-client", "getblockchaininfo", "block")
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
		"/nfs/bin/lambda-client", "getmininginfo", kind)
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
