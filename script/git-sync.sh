#!/usr/bin/env bash

{
  sleep 10
  echo 'HTTP/1.0 200 OK'
  echo 'Content-Type: text/html'
  echo
  echo 'OK'
} | nc -l 18080 &

cd ~/github
git pull
rsync -a --delete --exclude=.git/ ~/github/ ~/Dropbox/ICFPC2019/github
git rev-parse HEAD > ~/nfs/GIT_COMMIT_ID
git rev-parse HEAD > ~/Dropbox/ICFPC2019/GIT_COMMIT_ID
wait
