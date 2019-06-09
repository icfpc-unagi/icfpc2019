// Unagi launcher.

package main

import (
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
	"os/exec"
	"path"
	"strings"

	homedir "github.com/mitchellh/go-homedir"
)

func main() {
	pwd, err := os.Getwd()
	if err != nil {
		panic(fmt.Sprintf("failed to get current directory: %s", err))
	}

	pwd = strings.Replace(
		pwd, fmt.Sprintf(":%c", os.PathSeparator), "/", -1)
	pwd = strings.Replace(
		pwd, fmt.Sprintf("%c", os.PathSeparator), "/", -1)
	if !strings.HasPrefix(pwd, "/") {
		pwd = "/" + pwd
	}

	exe, err := os.Executable()
	if err != nil {
		panic(fmt.Sprintf("failed to get executable path: %s", err))
	}

	args := []string{
		"run",
		"-w", "/work",
		"-v", "/var/run/docker.sock:/var/run/docker.sock",
		"-v", pwd + ":/work",
		"-v", "/:/host",
		"-v", getCacheDirectory() + ":/root/.cache/icfpc2019",
		"-e", "HOST_PWD=" + pwd,
		"-e", "HOST_LAUNCHER=" + exe,
		"--privileged",
		"--pid=host",
		"--rm", "-it",
		getDockerImage(),
	}
	args = append(args, os.Args[1:]...)
	cmd := exec.Command("docker", args...)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	if err := cmd.Run(); err != nil {
		fmt.Fprintf(os.Stderr, "unagi command failed: %s\n", err)
		os.Exit(1)
	}
}

func getCacheDirectory() string {
	homeDir, err := homedir.Dir()
	if err != nil {
		panic(fmt.Sprintf("failed to home directory: %s", err))
	}
	cacheDir := path.Join(homeDir, ".cache", "icfpc2019")
	if err := os.MkdirAll(path.Join(cacheDir, "icfpc2019"), 0755); err != nil {
		panic(fmt.Sprintf("failed to create %s directory: %s",
			path.Join(cacheDir, "icfpc2019"), err))
	}
	return cacheDir
}

// getDockerImage returns an image name.
func getDockerImage() string {
	url := "https://storage.googleapis.com/unagi2019-public/hash/docker-master"
	resp, err := http.Get(url)
	if err != nil {
		panic(fmt.Sprintf("failed to get image information: %s", err))
	}
	defer resp.Body.Close()
	data, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		panic(fmt.Sprintf("failed to receive image information: %s", err))
	}
	return "unagi2019/image:" + strings.TrimSpace(string(data))
}
