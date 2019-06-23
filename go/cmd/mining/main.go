package main

import (
	"bytes"
	"encoding/json"
	"flag"
	"fmt"
	"io/ioutil"
	"math"
	"net/http"
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
			notify("#mining", fmt.Sprintf(
				"Block number %d is already mined.", blockNumber))
			time.Sleep(10 * time.Second)
			return nil
		}
		notify("#mining", fmt.Sprintf("Starting block number: %d", blockNumber))
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
		var solverErr error

		wg := sync.WaitGroup{}
		wg.Add(1)
		go func() {
			defer wg.Done()
			if output, err := generatePuzzle(); err != nil {
				solverErr = err
			} else {
				puzzleOutput = output
			}
			fmt.Fprintf(os.Stderr, "Puzzle solver finished.\n")
		}()
		wg.Add(1)
		go func() {
			defer wg.Done()
			if output, err := solveTask(); err != nil {
				solverErr = err
			} else {
				taskOutput = output
			}
			fmt.Fprintf(os.Stderr, "Task solver finished.\n")
		}()
		wg.Wait()

		if solverErr != nil {
			return solverErr
		}

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
			[]byte(taskOutput), 0644); err != nil {
			return errors.Errorf("failed to write task output: %s", err)
		}
		ioutil.WriteFile(
			path.Join(submitDir, "task_input.desc"), []byte(task), 0644)
		ioutil.WriteFile(
			path.Join(submitDir, "puzzle_input.cond"), []byte(puzzle), 0644)
		command := fmt.Sprintf("/nfs/bin/lambda-client submit %d %s %s",
			blockNumber,
			path.Join(submitDir, "task.sol"),
			path.Join(submitDir, "puzzle.desc"))
		if err := ioutil.WriteFile(
			path.Join(submitDir, "command.sh"),
			[]byte(command), 0644); err != nil {
			return errors.Errorf("failed to write command: %s", err)
		}
		notify("#mining", fmt.Sprintf(
			"Going to run `%s` in 60 seconds...", command))
		time.Sleep(time.Minute)
		stdout, stderr, err := execute("bash", "-c", command)
		fmt.Fprintf(os.Stderr, "%s\n", strings.TrimSpace(stdout))
		fmt.Fprintf(os.Stderr, "%s\n", strings.TrimSpace(stderr))
		notify("#mining", fmt.Sprintf(
			"Command result:\n- STDOUT: %s\n- STDERR: %s\n- Result: %s",
			strings.TrimSpace(stdout), strings.TrimSpace(stderr), err))
		if err != nil {
			return err
		}
		os.MkdirAll(fmt.Sprintf("/nfs/lock/%d", blockNumber), 0755)
		notify("#mining", fmt.Sprintf(
			"Successfully mined for block number %d\nFor more details, see %s.",
			blockNumber, submitDir))
		return nil
	}(); err != nil {
		notify("#mining", fmt.Sprintf("@channel Mining failed %+v", err))
		notify("#general", "@channel FAILED TO MINE!!! HELP ME!!!")
		panic(fmt.Sprintf("%+v", err))
	}
	fmt.Fprintf(os.Stderr, "Mining successfully finished.\n")
}

func generatePuzzle() (string, error) {
	type Program struct {
		Command string
		Output  string
		Score   int64
	}

	programs := []*Program{
		&Program{
			Command: "/nfs/programs/puzzle-003 ${PUZZLE_FILE} /dev/stdout",
		},
		&Program{
			Command: "/nfs/programs/puzzle-003 ${PUZZLE_FILE} /dev/stdout",
		},
		&Program{
			Command: "/nfs/programs/puzzle-003 ${PUZZLE_FILE} /dev/stdout",
		},
		&Program{
			Command: "/nfs/programs/puzzle-001 ${PUZZLE_FILE} /dev/stdout",
		},
		&Program{
			Command: "/nfs/programs/puzzle-001 ${PUZZLE_FILE} /dev/stdout",
		},
		&Program{
			Command: "/nfs/programs/puzzle-001 ${PUZZLE_FILE} /dev/stdout",
		},
		&Program{
			Command: "/nfs/programs/puzzle-002 ${PUZZLE_FILE} /dev/stdout",
		},
		&Program{
			Command: "/nfs/programs/puzzle-002 ${PUZZLE_FILE} /dev/stdout",
		},
		&Program{
			Command: "/nfs/programs/puzzle-002 ${PUZZLE_FILE} /dev/stdout",
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
				fmt.Fprintf(os.Stderr, "%v\n", err)
				return
			}
			solution := path.Join(dir, "output.desc")
			ioutil.WriteFile(solution, []byte(program.Output), 0644)
			stdout, stderr, err := execute(
				"bash", "-c",
				"/nfs/bin/puzzle_checker ${PUZZLE_FILE} "+solution)
			if err != nil {
				fmt.Fprintf(os.Stderr, "%s\n", strings.TrimSpace(stderr))
				fmt.Fprintf(
					os.Stderr, "Scorerer failed: %s: %s\n",
					program.Command, err)
				return
			}
			if regexp.MustCompile(`Success!`).MatchString(stdout) {
				program.Score = 1
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
		return "", errors.New("no puzzle generated")
	}
	message := ""
	for idx, program := range programs {
		message += fmt.Sprintf("#%d score=%d (%s)\n",
			idx, program.Score, program.Command)
	}
	fmt.Fprintf(os.Stderr, "%s\n", strings.TrimSpace(message))
	notify("#mining", fmt.Sprintf("Puzzle results:\n%s", message))
	return programs[0].Output, nil
}

func solveTask() (string, error) {
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
		&Program{
			Command: "/nfs/programs/wata-extend ${TASK_FILE}",
		},
		&Program{
			Command: "/nfs/programs/wata-extend ${TASK_FILE} all",
		},
		// &Program{
		// 	Command: "/nfs/programs/extend-fast ${TASK_FILE}",
		// },
		// &Program{
		// 	Command: "/nfs/programs/extend-fast ${TASK_FILE} all",
		// },
		&Program{
			Command: "/nfs/programs/extend-optimize ${TASK_FILE}",
		},
		&Program{
			Command: "/nfs/programs/chokudai-012 ${TASK_FILE}",
		},
		&Program{
			Command: "/nfs/programs/extend-optimize3 ${TASK_FILE}",
		},
		&Program{
			Command: "/nfs/programs/extend-optimize3 ${TASK_FILE} 2",
		},
		&Program{
			Command: "/nfs/programs/binary-search ${TASK_FILE}",
		},
		&Program{
			Command: "/nfs/programs/binary-search ${TASK_FILE} 2",
		},
		&Program{
			Command: "/nfs/programs/reverse ${TASK_FILE}",
		},
		&Program{
			Command: "/nfs/programs/reverse ${TASK_FILE} 2",
		},
		&Program{
			Command: "/nfs/programs/mining ${TASK_FILE}",
		},
		&Program{
			Command: "/nfs/programs/mining ${TASK_FILE} 2",
		},
		&Program{
			Command: "/nfs/programs/akiba-opt2 ${TASK_FILE} ''",
		},
		&Program{
			Command: "/nfs/programs/akiba-opt2 ${TASK_FILE} '' 2",
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
				fmt.Fprintf(os.Stderr, "%v\n", err)
				return
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
		return "", errors.New("task is not solved")
	}
	message := ""
	for idx, program := range programs {
		message += fmt.Sprintf("#%d score=%d (%s)\n",
			idx, program.Score, program.Command)
	}
	fmt.Fprintf(os.Stderr, "%s\n", strings.TrimSpace(message))
	notify("#mining", fmt.Sprintf("Mining results:\n%s", message))
	return programs[0].Output, nil
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

func notify(channel string, text string) {
	type Payload struct {
		Channel  string `json:"channel"`
		Username string `json:"username"`
		Text     string `json:"text"`
	}
	jsonBuf, err := json.Marshal(Payload{
		Channel:  channel,
		Username: "miningbot",
		Text:     text,
	})
	if err != nil {
		panic(err)
	}
	req, err := http.NewRequest(
		"POST",
		"https://hooks.slack.com/services/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",
		bytes.NewBuffer(jsonBuf),
	)
	if err != nil {
		panic(err)
	}
	req.Header.Set("Content-Type", "application/json")

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		panic(err)
	}
	defer resp.Body.Close()
}
