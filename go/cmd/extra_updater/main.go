package main

import (
	"context"
	"flag"
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"os/exec"
	"time"

	"github.com/imos/icfpc2019/go/util/apiutil"
	"github.com/imos/icfpc2019/go/util/pb"
	"github.com/pkg/errors"
)

func main() {
	flag.Parse()

	ctx := context.Background()
	for updateProblemImage(ctx) {
	}
	for updateSolutionImage(ctx) {
	}
}

func updateSolutionImage(ctx context.Context) bool {
	resp, err := apiutil.Call(ctx, &pb.Api_Request{
		AcquireSolutionExtra: &pb.Api_Request_AcquireSolutionExtra{},
	})
	if err != nil {
		log.Fatal(err)
	}
	fmt.Fprintf(os.Stderr, "AcquireSolutionExtra: %s\n", resp)
	id := resp.GetAcquireSolutionExtra().GetSolutionId()
	desc := resp.GetAcquireSolutionExtra().GetProblemDataBlob()
	sol := resp.GetAcquireSolutionExtra().GetSolutionDataBlob()
	modified := resp.GetAcquireSolutionExtra().GetSolutionDataModified()
	if desc == nil {
		return false
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

	solfile, err := ioutil.TempFile("", "sol")
	if err != nil {
		log.Fatal(err)
	}
	err = ioutil.WriteFile(solfile.Name(), sol, 0644)
	if err != nil {
		log.Fatal(err)
	}
	defer os.Remove(solfile.Name())

	pngfile, err := ioutil.TempFile("", "png")
	if err != nil {
		log.Fatal(err)
	}
	defer os.Remove(descfile.Name())

	fmt.Fprintf(os.Stderr, "Run sim -g\n")
	err = execute("/nfs/bin/sim", descfile.Name(), solfile.Name(), "-g", pngfile.Name())
	if err != nil {
		fmt.Fprintf(os.Stderr, "%v", err)
	}

	png, err := ioutil.ReadFile(pngfile.Name())
	if err != nil {
		log.Fatal(err)
	}
	resp2, err := apiutil.Call(ctx, &pb.Api_Request{
		UpdateSolutionExtra: &pb.Api_Request_UpdateSolutionExtra{
			SolutionId:           id,
			SolutionDataImage:    png,
			SolutionDataModified: modified,
		},
	})
	if err != nil {
		log.Fatal(err)
	}
	fmt.Fprintf(os.Stderr, "UpdateSolutionExtra: %s\n", resp2)

	fmt.Fprintf(os.Stderr, "Written for solution %d (%d bytes)\n", id, len(png))
	return true
}

func updateProblemImage(ctx context.Context) bool {
	resp, err := apiutil.Call(ctx, &pb.Api_Request{
		AcquireProblemExtra: &pb.Api_Request_AcquireProblemExtra{},
	})
	if err != nil {
		log.Fatal(err)
	}
	fmt.Fprintf(os.Stderr, "AcquireProblemExtra: %s\n", resp)
	id := resp.GetAcquireProblemExtra().GetProblemId()
	desc := resp.GetAcquireProblemExtra().GetProblemDataBlob()
	if desc == nil {
		return false
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
	err = execute("/nfs/bin/render_task", descfile.Name(), pngfile.Name())
	if err != nil {
		fmt.Fprintf(os.Stderr, "%v", err)
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
	return true
}

func execute(name string, args ...string) (err error) {
	cmd := exec.Command(name, args...)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

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
