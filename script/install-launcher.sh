#!/bin/bash

set -eu

bootstrap_directory='/usr/local/bin'
path="${PATH}:"
for candidate in ~/.local/bin ~/bin; do
    if [ "${path//"${candidate}":/}" != "${path}" ]; then
        if [ ! -e "${candidate}" ]; then
            if ! ask "${candidate} does not exist. Do you want to create it?"; then
                continue
            fi
            mkdir -p "${candidate}"
        fi
        bootstrap_directory="${candidate}"
        break
    fi
done

echo "Installing unagi launcher to ${bootstrap_directory}/unagi..." >&2

TMPDIR="${TMPDIR:=/tmp}"

cat <<'EOM' >"$TMPDIR/unagi.sh"
#!/bin/bash

os="$(uname | tr '[A-Z]' '[a-z]')"
file="$HOME/.cache/icfpc2019/unagi"
mkdir -p "$(dirname "${file}")"
curl --silent -z "${file}" -o "${file}.tmp" \
    "https://storage.googleapis.com/icfpc-public-data/bin/launcher-${os}" &&
if [ -f "${file}.tmp" ]; then
    chmod +x "${file}.tmp"
    mv "${file}.tmp" "${file}"
fi
exec "${file}" "$@"
EOM

cat <<EOM >"$TMPDIR/launcher-setup.sh"
#!/bin/bash

cat "$TMPDIR/unagi.sh" > "${bootstrap_directory}/unagi"
chmod 755 "${bootstrap_directory}/unagi"
EOM

if [ "${bootstrap_directory}" == '/usr/local/bin' ]; then
    sudo bash "$TMPDIR/launcher-setup.sh"
else
    bash "$TMPDIR/launcher-setup.sh"
fi
