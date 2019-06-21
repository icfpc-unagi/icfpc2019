#!/usr/bin/env bash

poll() {
  {
    sleep 10
    echo 'HTTP/1.0 200 OK'
    echo 'Content-Type: text/html'
    echo
    echo 'OK'
  } | nc -CNl 18080
}

cd ~/github
while :; do
  poll </dev/null &
  sleep 5 &
  git pull
  rsync -a --delete --exclude=.git/ ~/github/ ~/Dropbox/ICFPC2019/github
  git rev-parse HEAD > ~/info/GIT_COMMIT_ID
  unagi --root ~/info --bare gsutil cp -r /work/* gs://icfpc2019-asia/info/
  git rev-parse HEAD > ~/Dropbox/ICFPC2019/GIT_COMMIT_ID
  wait
done
