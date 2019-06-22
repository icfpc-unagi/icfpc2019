package main

import (
	"context"
	"flag"
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"os/exec"
	"path"
	"time"

	"github.com/imos/icfpc2019/go/util/apiutil"
	"github.com/imos/icfpc2019/go/util/pb"
	"github.com/pkg/errors"
)

func main() {
	flag.Parse()

	ctx := context.Background()
	for {
		resp, err := apiutil.Call(ctx, &pb.Api_Request{
			AcquireProblemExtra: &pb.Api_Request_AcquireProblemExtra{},
		})
		if err != nil {
			log.Fatal(err)
		}
		fmt.Fprintf(os.Stderr, "AcquireProblemExtra: %s\n", resp)
		id := resp.GetAcquireProblemExtra().GetProblemId()
		desc := resp.GetAcquireProblemExtra().GetProblemDataBlob()
		if resp.AcquireProblemExtra == nil {
			fmt.Fprintf(os.Stderr, "Nothing to do...\n")
			time.Sleep(60 * time.Second)
			continue
		}

		descfile, err := ioutil.TempFile("", "desc")
		if err != nil {
			log.Fatal(err)
		}
		err = ioutil.WriteFile(descfile.Name(), desc, 0644)
		if err != nil {
			log.Fatal(err)
		}
		defer os.Remove(descfile.Name())

		pngfile, err := ioutil.TempFile("", "png")
		if err != nil {
			log.Fatal(err)
		}
		defer os.Remove(descfile.Name())

		fmt.Fprintf(os.Stderr, "Run render_task\n")
		_, _, err = execute("/nfs/bin/render_task", descfile.Name(), pngfile.Name())
		if err != nil {
			log.Fatal(err)
		}

		png, err := ioutil.ReadFile(pngfile.Name())
		if err != nil {
			log.Fatal(err)
		}
		resp2, err := apiutil.Call(ctx, &pb.Api_Request{
			UpdateProblemExtra: &pb.Api_Request_UpdateProblemExtra{
				ProblemId:        id,
				ProblemDataImage: png,
			},
		})
		if err != nil {
			log.Fatal(err)
		}
		fmt.Fprintf(os.Stderr, "UpdateProblemExtra: %s\n", resp2)

		fmt.Fprintf(os.Stderr, "Written for problem %d (%d bytes)\n", id, len(png))
	}
}

func execute(
	name string, args ...string,
) (stdout string, stderr string, err error) {
	dir, err := ioutil.TempDir("", "upd")
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
