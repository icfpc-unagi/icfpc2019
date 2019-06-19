// Unagi launcher.

package main

import (
	"flag"
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
	"os/exec"
	"os/user"
	"path"
	"regexp"
	"strings"

	"github.com/mattn/go-isatty"
	homedir "github.com/mitchellh/go-homedir"
)

var tty = flag.Bool("tty", false, "Enable tty.")
var image = flag.String("image", "", "Image to use.")
var force = flag.Bool("force", false, "Mount the current directory")

func main() {
	flag.Parse()

	rootDir, relativeDir := getUnagiDirectory()

	exe, err := os.Executable()
	if err != nil {
		panic(fmt.Sprintf("failed to get executable path: %s", err))
	}

	user := func() string {
		if u := os.Getenv("HOST_USER"); u != "" {
			return u
		} else if u := os.Getenv("USER"); u != "" {
			return u
		} else if u, err := user.Current(); err != nil {
			return u.Username
		} else if u, err := exec.Command("id", "-un").Output(); err != nil {
			return string(u)
		}
		panic("failed to guess user")
	}()
	user = strings.ToLower(user)
	if !regexp.MustCompile(`^[a-z][a-z0-9\-]{1,16}$`).MatchString(user) {
		panic(fmt.Sprintf("inavlid user: %s", user))
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
		"-e", "HOST_USER=" + user,
		"--privileged",
		"--pid=host",
		"--rm", "-i",
	}
	if os.Getenv("TERM") == "xterm-256color" {
		args = append(args, "-e", "TERM=xterm-256color")
	}
	if (isatty.IsTerminal(os.Stdin.Fd()) &&
		isatty.IsTerminal(os.Stdout.Fd()) &&
		isatty.IsTerminal(os.Stderr.Fd())) ||
		(isatty.IsCygwinTerminal(os.Stdin.Fd()) &&
			isatty.IsCygwinTerminal(os.Stdout.Fd()) &&
			isatty.IsCygwinTerminal(os.Stderr.Fd())) {
		args = append(args, "-t")
	}
	args = append(args, getDockerImage())
	args = append(args, flag.Args()...)
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
	if *force {
		return
	}

	for {
		if _, err := os.Stat(
			path.Join(rootDir, "UNAGI_REPOSITORY")); err == nil {
			return
		}
		if path.Dir(rootDir) == rootDir {
			panic("unagi command must be run under the team repository: " +
				getCurrentDirectory())
		}
		relativeDir = path.Join(path.Base(rootDir), relativeDir)
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
	if *image != "" {
		return "unagi2019/image:" + *image
	}

	url := "https://storage.googleapis.com/icfpc-public-data/hash/docker-master"
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
