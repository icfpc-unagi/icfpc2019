#!/usr/bin/env bash
# Usage: push-docker-image.sh tag

set -eu

TAG="$1"
image_id="$(docker image inspect --format='{{.Id}}' "unagi2019/image:${TAG}")"
image_id="${image_id##sha256:}"
image_id="${image_id:0:8}"
tag="${TAG}-$(date '+%Y%m%d')-${image_id}"
echo "Pushing ${tag}..." >&2
docker tag "unagi2019/image:${TAG}" "unagi2019/image:${tag}"
docker push "unagi2019/image:${tag}"
tmpfile=`mktemp`
echo -n "${tag}" > "${tmpfile}"
gsutil cp "${tmpfile}" "gs://unagi2019-public/hash/docker-${TAG}"
rm "${tmpfile}"
