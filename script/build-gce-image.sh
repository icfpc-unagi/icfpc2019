#!/usr/bin/env bash

source "$(dirname "${BASH_SOURCE}")/../bin/imosh" || exit 1
DEFINE_string project 'icfpc-primary' 'Project ID.'
DEFINE_string zone 'asia-northeast1-a' 'Zone.'
DEFINE_string instance 'image' 'Instance ID.'
eval "${IMOSH_INIT}"

set -eu

do_start() {
  LOG INFO 'Starting instance...'
  gcloud compute --project="${FLAGS_project}" instances create \
      "${FLAGS_instance}" \
      --zone="${FLAGS_zone}" --machine-type=custom-1-4096 --subnet=default \
      --network-tier=PREMIUM --maintenance-policy=MIGRATE \
      --service-account=289881194472-compute@developer.gserviceaccount.com \
      --scopes=https://www.googleapis.com/auth/devstorage.read_only,https://www.googleapis.com/auth/logging.write,https://www.googleapis.com/auth/monitoring.write,https://www.googleapis.com/auth/servicecontrol,https://www.googleapis.com/auth/service.management.readonly,https://www.googleapis.com/auth/trace.append \
      --image=ubuntu-1804-bionic-v20190612 --image-project=ubuntu-os-cloud \
      --boot-disk-size=20GB --boot-disk-type=pd-standard \
      --boot-disk-device-name="${FLAGS_instance}"
  LOG INFO 'Successfully created an instance.'
}

do_wait() {
  for i in `seq 100`; do
    if do_ssh true 2>/dev/null; then
      LOG INFO "Confirmed ${FLAGS_instance} is running."
      return
    fi
    sleep 5
  done
  LOG FATAL "Failed to wait for instance ${FLAGS_instance}"
}

do_ssh() {
  gcloud compute --project="${FLAGS_project}" ssh "${FLAGS_instance}" \
      --zone="${FLAGS_zone}" -- "$@"
}

do_scp() {
  gcloud compute --project="${FLAGS_project}" scp --zone="${FLAGS_zone}" \
      --scp-flag=-q --recurse "$@"
}

do_setup() {
  do_scp "$(dirname "${BASH_SOURCE}")/../bin" "${FLAGS_instance}":/tmp/
  do_scp "$(dirname "${BASH_SOURCE}")/../script" "${FLAGS_instance}":/tmp/

  if [ "$#" -ge 1 ]; then
    targets=("$@")
  else
    targets=(
        setup_environment
        setup_sudoers
        install_git_lfs
        setup_systemd
        setup_grub
        install_gcloud
        install_docker
        # setup_guest depends on install_docker.
        setup_guest
        install_unagi
        clean
        setup_swapfile
    )
  fi

  for target in "${targets[@]}"; do
    do_ssh sudo bash /tmp/script/build-gce-image-commands.sh \
        --logtostderr --password="${UNAGI_PASSWORD}" \
        "${target}"
  done
}

do_all() {
  do_start
  do_wait
  do_setup
  do_ssh shutdown now || true
}

if [ "$#" -eq 0 ]; then
    LOG FATAL 'command must be given'
fi
command="$1"
shift
"do_${command}" "$@"
