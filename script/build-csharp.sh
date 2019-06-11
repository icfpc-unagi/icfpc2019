#!/usr/bin/env bash

set -eu

mkdir -p build
for file in `find . -name '*.cs'`; do
    echo "Building $file..." >&2;
    name="$(basename "${file%.cs}")"
    mcs -out:"build/$name" "$file";
    cat <<EOM > "build/${name}"
#!/usr/bin/env bash
exec unagi mono "\$(dirname "\${BASH_SOURCE}")/${name}.exe"
EOM
    chmod +x "build/${name}"
done
