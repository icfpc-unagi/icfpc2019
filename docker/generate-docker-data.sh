#!/usr/bin/env bash

cd "$(dirname "${BASH_SOURCE}")/data"

mkdir -p '../../build/docker'
for file in *; do
    if [[ "${file}" =~ \.encrypted$ ]]; then
        cat "${file}" | decrypt > "../../build/docker/${file%.encrypted}"
    else
        cp "${file}" "../../build/docker/${file}"
    fi
done
