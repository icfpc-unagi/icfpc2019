#!/bin/sh
set -e

MINING_ROOT="/Users/akiba/Dropbox/ICFPC2019/mining"
BLOCK=$(./lambda-cli.py getblockchaininfo block)

TASK_SOL_PATH="${MINING_ROOT}/out/task_${BLOCK}.sol"
PUZZLE_SOL_PATH="${MINING_ROOT}/out/puzzle_${BLOCK}.desc"

ls -alFh ${TASK_SOL_PATH}
ls -alFh ${PUZZLE_SOL_PATH}

echo ""
echo "OK?:"
echo "  " python lambda-cli.py submit ${BLOCK} ${TASK_SOL_PATH} ${PUZZLE_SOL_PATH}
echo 

while true; do
    echo "Yes or No:"

    read answer

    case $answer in
        yes)
            python lambda-cli.py submit ${BLOCK} ${TASK_SOL_PATH} ${PUZZLE_SOL_PATH}
            break
            ;;
        no)
            echo "Abort"
            break
            ;;
        *)
            echo "Unrecognized: $answer"
            ;;
    esac
done

