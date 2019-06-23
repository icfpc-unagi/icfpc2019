#!/usr/bin/env bash

source /repo/bin/imosh || exit 1

set -eu
solution_checker="$1"

# Example 1
CHECK [ "$(
    "$1" /repo/data/part-1-examples/example-01.desc \
        /repo/data/part-1-examples/example-01-1.sol)" == \
    "$(
        echo 'Success! '
        echo 'Your solution took 48 time units. '
    )" ]

# Example 2
CHECK [ "$(
    "$1" /repo/data/part-2-teleports-examples/example-02.desc \
        /repo/data/part-2-teleports-examples/example-02-1.sol)" == \
    "$(
        echo 'Success! '
        echo 'Your solution took 60 time units. '
    )" ]

# Example 3
CHECK [ "$(
    "$1" /repo/data/part-3-clones-examples/example-03.desc \
        /repo/data/part-3-clones-examples/example-03-1.sol)" == \
    "$(
        echo 'Success! '
        echo 'Your solution took 28 time units. '
    )" ]

# Task is empty.
CHECK [ "$(
    "$1" /dev/null \
        /repo/data/part-1-examples/example-01-1.sol
    )" == \
    "$(
        echo 'Error! task data is empty.'
    )" ]

# Solution is empty.
CHECK [ "$(
    "$1" /repo/data/part-3-clones-examples/example-03.desc \
        /dev/null
    )" == \
    "$(
        echo 'Error! solution data is empty.'
    )" ]

# Solution is invalid.
echo 'invalid string' > "$TMPDIR/invalid_string"
CHECK [ "$(
    "$1" /repo/data/part-3-clones-examples/example-03.desc \
        "$TMPDIR/invalid_string"
    )" == \
    "$(
        echo 'Cannot check: some parts of the input are missing or malformed'
    )" ]

# Example 4.
CHECK [ "$(
    "$1" /repo/data/purchasing-examples/example-04.desc \
        /repo/data/purchasing-examples/example-04-1.sol \
        /repo/data/purchasing-examples/example-04.buy
    )" == \
    "$(
        echo 'Success! '
        echo 'Your solution took 28 time units. '
    )" ]

# With insufficient boosters.
CHECK [ "$(
    "$1" /repo/data/purchasing-examples/example-04.desc \
        /repo/data/purchasing-examples/example-04-1.sol
    )" == \
    "$(
        echo 'Failed: No such booster available, tried at location (0,9)'
    )" ]
CHECK [ "$(
    "$1" /repo/data/purchasing-examples/example-04.desc \
        /repo/data/purchasing-examples/example-04-1.sol \
        /dev/null
    )" == \
    "$(
        echo 'Failed: No such booster available, tried at location (0,9)'
    )" ]

# With invalid boosters.
CHECK [ "$(
    "$1" /repo/data/part-1-examples/example-01.desc \
        /repo/data/part-1-examples/example-01-1.sol \
        "$TMPDIR/invalid_string"
    )" == \
    "$(
        echo 'Success! '
        echo 'Your solution took 48 time units. '
    )" ]

echo 'All tests passed.' >&2
