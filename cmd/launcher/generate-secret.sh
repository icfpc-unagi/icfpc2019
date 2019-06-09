#!/usr/bin/env bash

set -eu

cat <<EOM > 'secret.go'
package main

func init() {
	password = "${UNAGI_PASSWORD}"
	bucket = "${UNAGI_PUBLIC_BUCKET}"
}
EOM
