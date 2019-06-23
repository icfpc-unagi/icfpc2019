#!/usr/bin/env bash

source "$(dirname "${BASH_SOURCE}")/../bin/imosh" || exit 1
eval "${IMOSH_INIT}"

curl --silent -L -o solutions.zip \
    https://dashboard.sx9.jp/ytueijprkwrkaqzh/download
unzip -q -t solutions.zip

if unzip -l solutions.zip -x 'prob-*.sol' 2>/dev/null >/dev/null; then
    LOG FATAL 'non prob-*.sol files exist'
fi

export TMPDIR=`mktemp -d`
cp solutions.zip "$TMPDIR/"
pushd "$TMPDIR" >/dev/null
unzip -q solutions.zip >/dev/null

for file in prob-*.sol; do
    echo "${file%.sol}"
done | xargs -P 12 '-I{}' bash -c 'validate /nfs/github/data/*/{}.desc {}.sol'
