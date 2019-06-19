#!/usr/bin/env bash
# Usage: bash remote.sh [options]

source "$(dirname "${BASH_SOURCE}")/../bin/imosh" || exit 1
DEFINE_string password '' 'Unagi password.'
eval "${IMOSH_INIT}"

set -eu

do_setup_environment() {
    LOG INFO 'Updating /etc/environment...'
    cat <<EOM >/etc/environment
PATH="/usr/local/google-cloud-sdk/bin:/usr/local/cuda/bin:/go/bin:/usr/local/go/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/snap/bin:/sbin:/bin:/usr/games:/usr/local/games"
UNAGI_PASSWORD="${FLAGS_password}"
BOTO_CONFIG="/dev/null"
EOM
}

do_setup_sudoers() {
    sed -i.bak -e 's%.*secure_path.*%Defaults secure_path="/usr/local/google-cloud-sdk/bin:/usr/local/cuda/bin:/go/bin:/usr/local/go/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/snap/bin:/sbin:/bin:/usr/games:/usr/local/games"%g' /etc/sudoers
    sed -i.bak -e 's#^%sudo.*#%sudo ALL=NOPASSWD: ALL#g' /etc/sudoers
}

do_setup_guest() {
    if ! id 'guest'; then
        adduser --disabled-password --gecos '' 'guest'
    fi
    usermod -G docker,ubuntu,sudo guest
    mkdir -p /home/guest/.ssh
    rm -rf /home/guest/.ssh/authorized_keys
    for user in imos iwiwi chokudai toslunar wata-orz sulume ninetan; do
        curl "https://github.com/${user}.keys" \
            >> /home/guest/.ssh/authorized_keys
    done
}

do_install_git_lfs() {
    apt-get update -qqy
    apt-get install -qqy git-lfs
}

do_setup_systemd() {
    # Disable apt daily services to avoid apt conflict.
    systemctl mask apt-daily.service
    systemctl mask apt-daily.timer
    systemctl mask apt-daily-upgrade.service
    systemctl mask apt-daily-upgrade.timer
}

do_setup_grub() {
    # Update grub to enable cgroups to limit swap memory.
    if ! grep swapaccount /etc/default/grub.d/50-cloudimg-settings.cfg; then
        sed -i.bak -e 's%^GRUB_CMDLINE_LINUX="%GRUB_CMDLINE_LINUX="cgroup_enable=memory swapaccount=1 %' \
            /etc/default/grub.d/50-cloudimg-settings.cfg
        update-grub
    fi
}

do_setup_swapfile() {
    # Prepare swap file.
    mkdir -p /var/vm
    if [ ! -f /var/vm/sentinel ]; then
        fallocate -l 2G /var/vm/sentinel
        chmod 600 /var/vm/sentinel
        mkswap /var/vm/sentinel || true
        swapon /var/vm/sentinel || true
        if ! grep /var/vm/sentinel /etc/fstab; then
            echo '/var/vm/sentinel swap swap defaults,pri=0 0 0' >> /etc/fstab
        fi
    fi
    if [ ! -f /var/vm/swapfile ]; then
        fallocate -l 16M /var/vm/swapfile
        chmod 600 /var/vm/swapfile
        mkswap /var/vm/swapfile || true
        swapon /var/vm/swapfile || true
        if ! grep /var/vm/swapfile /etc/fstab; then
            echo '/var/vm/swapfile swap swap defaults,pri=1 0 0' >> /etc/fstab
        fi
    fi
}

do_install_cxx() {
    # Install C++ tools.
    apt-get update -qqy
    apt-get install -qqy g++ clang-format cmake
}

do_install_gcloud() {
    # Install Google Cloud SDK.
    echo "deb http://packages.cloud.google.com/apt" \
        "cloud-sdk-$(lsb_release -c -s) main" \
        > /etc/apt/sources.list.d/google-cloud-sdk.list
    curl https://packages.cloud.google.com/apt/doc/apt-key.gpg | apt-key add -
    apt-get update -qqy
    apt-get install -qqy google-cloud-sdk google-cloud-sdk-app-engine-go \
        google-cloud-sdk-app-engine-python google-cloud-sdk-app-engine-go \
        google-cloud-sdk-datastore-emulator
}

do_install_docker() {
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
    add-apt-repository "$(echo \
        "deb [arch=amd64] https://download.docker.com/linux/ubuntu" \
        "$(lsb_release -cs)" stable)"
    apt-get update -qqy
    apt-get install -qyy docker-ce docker-ce-cli containerd.io
    curl -L https://github.com/docker/compose/releases/download/1.24.0/docker-compose-`uname -s`-`uname -m` -o /usr/local/bin/docker-compose
    chmod +x /usr/local/bin/docker-compose
    cat <<'EOM' >/etc/docker/daemon.json
{
    "registry-mirrors": ["https://mirror.gcr.io"]
}
EOM
}

do_install_unagi() {
    bash "$(dirname "${BASH_SOURCE}")/install-launcher.sh"
}

do_clean() {
    apt-get -y autoremove
    apt-get -y clean
}

do_docker_pull() {
    sudo -H docker login --username unagi2019 --password "${FLAGS_password}"
    sudo -H -u guest \
        docker login --username unagi2019 --password "${FLAGS_password}"
    sudo -H docker pull unagi2019/image:master
}

do_shutdown() {
    shutdown
}

for target in "$@"; do
    LOG INFO "Starting ${target}..."
    "do_${target}"
    LOG INFO "Successfully ${target} finished."
done
