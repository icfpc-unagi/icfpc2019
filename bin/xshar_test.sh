#!/usr/bin/env bash

set -eu
cd "$(dirname "${BASH_SOURCE}")"
XSHAR="$(pwd)/xshar"

cd `mktemp -d`

cat <<'EOM' > hello
#!/usr/bin/env bash
echo hello
EOM
chmod +x hello
"${XSHAR}" hello

if [ "$(./hello.shar)" != 'hello' ]; then
  echo 'hello.shar failed' >&2
  exit 1
fi
echo 'hello.shar passed' >&2

cat <<'EOM' > args
#!/usr/bin/env bash
echo "$@"
EOM
chmod +x args
"${XSHAR}" args

if [ "$(./args.shar)" != '' -o \
     "$(./args.shar foo)" != 'foo' -o \
     "$(./args.shar foo bar)" != 'foo bar' -o \
     "$(./args.shar --foo -- bar)" != '--foo -- bar' ]; then
  echo 'args.shar failed' >&2
  exit 1
fi
echo 'args.shar passed' >&2

cat <<'EOM' > execute
#!/usr/bin/env bash
exec "$@"
EOM
chmod +x execute
"${XSHAR}" execute hello

if [ "$(./execute.shar)" != '' -o \
     "$(./execute.shar echo hoge)" != 'hoge' ]; then
  echo 'execute.shar failed' >&2
  exit 1
fi
echo 'execute.shar passed' >&2
