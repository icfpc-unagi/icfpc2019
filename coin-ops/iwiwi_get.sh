#!/bin/sh
set -e

MINING_ROOT="/Users/akiba/Dropbox/ICFPC2019/mining"
BLOCK=$(./lambda-cli.py getblockchaininfo block)

TASK_PATH="${MINING_ROOT}/in/task_${BLOCK}.desc"
PUZZLE_PATH="${MINING_ROOT}/in/puzzle_${BLOCK}.cond"

./lambda-cli.py getmininginfo task > ${TASK_PATH}
./lambda-cli.py getmininginfo puzzle > ${PUZZLE_PATH}

echo ${TASK_PATH}
echo ${PUZZLE_PATH}
