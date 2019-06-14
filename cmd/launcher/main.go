// Unagi launcher.

package main

import (
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
	"os/exec"
	"path"
	"regexp"
	"strings"

	"github.com/mattn/go-isatty"
	homedir "github.com/mitchellh/go-homedir"
)

func main() {
	rootDir, relativeDir := getUnagiDirectory()

	exe, err := os.Executable()
	if err != nil {
		panic(fmt.Sprintf("failed to get executable path: %s", err))
	}

	args := []string{
		"run",
		"-w", "/work/" + toLinuxPath(relativeDir),
		"-v", "/var/run/docker.sock:/var/run/docker.sock",
		"-v", toLinuxPath(rootDir) + ":/work",
		"-v", "/:/host",
		"-v",
		toLinuxPath(getCacheDirectory("")) + ":/root/.cache/icfpc2019",
		"-v",
		toLinuxPath(getLocalCacheDirectory(rootDir, "cargo")) +
			":/usr/local/cargo/registry",
		"-v",
		toLinuxPath(getLocalCacheDirectory(rootDir, "go-pkg")) +
			":/go/pkg",
		"-v",
		toLinuxPath(getLocalCacheDirectory(rootDir, "go-build-cache")) +
			":/root/.cache/go-build",
		"-e", "HOST_PWD=" + toLinuxPath(getCurrentDirectory()),
		"-e", "HOST_LAUNCHER=" + toLinuxPath(exe),
		"--privileged",
		"--pid=host",
		"--rm", "-i",
	}
	if isatty.IsTerminal(os.Stdin.Fd()) ||
		isatty.IsCygwinTerminal(os.Stdin.Fd()) {
		args = append(args, "-t")
	}
	args = append(args, getDockerImage())
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

func getUnagiDirectory() (rootDir, relativeDir string) {
	rootDir, relativeDir = getCurrentDirectory(), "."
	for {
		if _, err := os.Stat(
			path.Join(rootDir, "UNAGI_REPOSITORY")); err == nil {
			return
		}
		if path.Dir(rootDir) == rootDir {
			panic("unagi command must be run under the team repository: " +
				getCurrentDirectory())
		}
		relativeDir = path.Join(relativeDir, path.Base(rootDir))
		rootDir = path.Dir(rootDir)
	}
}

func toLinuxPath(path string) string {
	path = strings.Replace(
		path, fmt.Sprintf("%c", os.PathSeparator), "/", -1)
	if m := regexp.MustCompile(`^(\w):/(.*)$`).FindStringSubmatch(
		path); m != nil && len(m) > 3 {
		path = "/" + m[1] + "/" + m[2]
	}
	return path
}

func getCacheDirectory(name string) string {
	homeDir, err := homedir.Dir()
	if err != nil {
		panic(fmt.Sprintf("failed to home directory: %s", err))
	}
	cacheDir := path.Join(homeDir, ".cache", "icfpc2019")
	if name != "" {
		cacheDir = path.Join(cacheDir, name)
	}
	if err := os.MkdirAll(cacheDir, 0755); err != nil {
		panic(fmt.Sprintf("failed to create %s directory: %s", cacheDir, err))
	}
	return cacheDir
}

func getLocalCacheDirectory(rootDir string, name string) string {
	cacheDir := path.Join(rootDir, ".cache")
	if name != "" {
		cacheDir = path.Join(cacheDir, name)
	}
	if err := os.MkdirAll(cacheDir, 0755); err != nil {
		panic(fmt.Sprintf("failed to create %s directory: %s", cacheDir, err))
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

func getCurrentDirectory() string {
	pwd, err := os.Getwd()
	if err != nil {
		panic(fmt.Sprintf("failed to get current directory: %s", err))
	}
	return pwd
}
