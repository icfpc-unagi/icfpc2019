#!/usr/bin/env bash
# $ bash script/build-gce-image.sh all

source "$(dirname "${BASH_SOURCE}")/../bin/imosh" || exit 1
DEFINE_string key ~/.ssh/ec2-oregon.pem 'SSH private key.'
DEFINE_string ip '' 'Instance IP address.'
eval "${IMOSH_INIT}"

internal::ssh() {
  ssh -i "${FLAGS_key}" "ubuntu@${FLAGS_ip}" "$@"
}

internal::scp() {
  scp -i "${FLAGS_key}" -rq "$@"
}

do_setup() {
  internal::scp "$(dirname "${BASH_SOURCE}")/../bin" \
      "ubuntu@${FLAGS_ip}":/tmp/
  internal::scp "$(dirname "${BASH_SOURCE}")/../script" \
      "ubuntu@${FLAGS_ip}":/tmp/
  internal::scp "$(dirname "${BASH_SOURCE}")/../docker" \
      "ubuntu@${FLAGS_ip}":/tmp/

  if [ "$#" -ge 1 ]; then
    targets=("$@")
  else
    targets=(
        setup_environment
        setup_sudoers
        install_git_lfs
        # setup_systemd
        # setup_grub
        install_gcloud
        install_docker
        # setup_guest depends on install_docker.
        setup_guest
        docker_pull
        install_unagi
        install_pem
        clean
        setup_swapfile
    )
  fi

  for target in "${targets[@]}"; do
    internal::ssh sudo bash /tmp/script/build-aws-image-commands.sh \
        --logtostderr --password="${UNAGI_PASSWORD}" \
        "${target}"
  done
}

if [ "$#" -eq 0 ]; then
    LOG FATAL 'command must be given'
fi
command="$1"
shift
"do_${command}" "$@"
