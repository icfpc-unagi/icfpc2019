#!/usr/bin/env bash

cd "$(dirname "${BASH_SOURCE}")/.."

set -eu

for file in bin/*_test.sh; do
  stdout=`mktemp`
  stderr=`mktemp`
  if bash "${file}" >"${stdout}" 2>"${stderr}"; then
    echo "[PASSED] ${file}" >&2
  else
    echo "[FAILED] ${file}" >&2
    echo "STDOUT:" >&2
    cat "${stdout}" >&2
    echo "STDERR:" >&2
    cat "${stderr}" >&2
  fi
done
