#!/usr/bin/env bash
# Usage: validate-solutions solutions.zip

source "$(dirname "${BASH_SOURCE}")/../bin/imosh" || exit 1
eval "${IMOSH_INIT}"

if [ "$#" != 1 ]; then
    imosh::help
    exit 1
fi

unzip -q -t "$1"
if unzip -l "$1" -x 'prob-*.sol' -x 'prob-*.buy' \
    2>/dev/null >/dev/null; then
    LOG FATAL 'non prob-*.sol prob-*.buy files exist'
fi

export TMPDIR=`mktemp -d`
cp "$1" "$TMPDIR/solutions.zip"
pushd "$TMPDIR" >/dev/null
unzip -q solutions.zip >/dev/null

for id in `seq 300`; do
    id="$(printf '%03d' "${id}")"
    if [ ! -f "prob-${id}.sol" ]; then
        LOG FATAL "prob-${id}.sol is missing"
    fi
done

cat <<'EOM' > validate.sh
#!/usr/bin/env bash

args=(/nfs/bin/solution_checker /repo/data/*/$1.desc $1.sol)
if [ -f "$1.buy" ]; then
    args+=("$1.buy")
fi

echo "$1:" `"${args[@]}"`
EOM

for file in prob-*.sol; do
    echo "${file%.sol}"
done | xargs -P "$(nproc)" -I'{}' bash ./validate.sh '{}'
