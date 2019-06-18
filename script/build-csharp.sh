#!/usr/bin/env bash

set -eu

mkdir -p build
for directory in *; do
    if [ ! -d "${directory}" ]; then
        continue
    fi
    case "${directory}" in
        .* | target ) searchable=0;;
        * ) searchable=1;;
    esac
    if (( ! searchable )); then
        continue
    fi
    for file in `find "${directory}" -name '*.cs'`; do
        echo "Building $file..." >&2;
        name="$(basename "${file%.cs}")"
        mcs -out:"build/$name.exe" "$file";
        cat <<EOM > "build/${name}"
#!/usr/bin/env bash
exec unagi mono "\$(dirname "\${BASH_SOURCE}")/${name}.exe"
EOM
        chmod +x "build/${name}"
    done
done
