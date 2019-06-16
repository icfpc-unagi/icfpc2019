FROM ubuntu:18.04

################################################################################
# Environment variables (required for installation)
# NOTE: environment variables not required for installation can be placed in
# `Configurations` section.
################################################################################

ENV DEBIAN_FRONTEND noninteractive
ENV APT_KEY_DONT_WARN_ON_DANGEROUS_USAGE=DontWarn

ARG UNAGI_PASSWORD
RUN [ "${UNAGI_PASSWORD}" != '' ]
ENV UNAGI_PASSWORD=$UNAGI_PASSWORD

ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/work/bin:/usr/local/cargo/bin:/usr/local/go/bin:$PATH
ENV GOROOT=/usr/local/go
ENV GOPATH=/go

################################################################################
# Installation
################################################################################

# Use GCP apt.
RUN sed -i.bak -e "s%http://archive.ubuntu.com/ubuntu/%http://asia-northeast1.gce.archive.ubuntu.com/ubuntu/%g" /etc/apt/sources.list

# Install fundamental tools.
RUN apt-get update -q && apt-get install -qy apt-utils curl && \
    apt-get clean -q && rm -rf /var/lib/apt/lists/*

# Do not exclude man pages & other documentation
RUN rm /etc/dpkg/dpkg.cfg.d/excludes
# Reinstall all currently installed packages in order to get the man pages back
RUN apt-get update -q && \
    dpkg -l | grep ^ii | cut -d' ' -f3 | \
        xargs apt-get install -qy --reinstall && \
    apt-get clean -q && rm -rf /var/lib/apt/lists/*

# Install C++.
RUN apt-get update -q && apt-get install -qy clang clang-format g++ && \
    apt-get clean -q && rm -rf /var/lib/apt/lists/*

# Install C#.
RUN apt-get update -q && apt-get install -qy gnupg ca-certificates && \
    apt-key adv --keyserver hkp://keyserver.ubuntu.com:80 \
        --recv-keys 3FA7E0328081BFF6A14DA29AA6A19B38D3D831EF && \
    echo "deb https://download.mono-project.com/repo/ubuntu stable-bionic main" \
        > /etc/apt/sources.list.d/mono-official-stable.list && \
    apt-get update -qy && apt-get install -qy mono-devel && \
    apt-get clean -q && rm -rf /var/lib/apt/lists/*

# Install Java.
RUN apt-get update && apt-get install -y default-jre default-jdk && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Install Bazel.
RUN apt-get update -q && apt-get install -qy unzip && \
    apt-get clean && rm -rf /var/lib/apt/lists/* && \
    curl -L -o installer \
    "https://github.com/bazelbuild/bazel/releases/download/0.26.1/bazel-0.26.1-installer-linux-x86_64.sh" && \
    chmod +x installer && ./installer && rm ./installer && \
    echo 'source /usr/local/lib/bazel/bin/bazel-complete.bash' > /etc/profile.d/99-bazel-complete.sh && \
    chmod +x /etc/profile.d/99-bazel-complete.sh

# Install Rust.
RUN set -eux; \
    curl -o rustup-init "https://static.rust-lang.org/rustup/archive/1.12.0/x86_64-unknown-linux-gnu/rustup-init"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain 1.35.0; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; cargo --version; rustc --version

# Install Go.
RUN curl -o go.tar.gz https://dl.google.com/go/go1.12.5.linux-amd64.tar.gz && \
    tar -xf go.tar.gz && \
    mv go /usr/local/ && \
    rm go.tar.gz && \
    mkdir -p /go/src && \
    echo 'GOROOT="/usr/local/go"' >> /etc/environment && \
    echo 'GOPATH="/go"' >> /etc/environment

# Install scripts (python, php, ruby).
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
        php-cli php-mysql php-curl php-pear python python-pip ruby && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Install other useful tools.
RUN apt-get update && apt-get install -y \
        build-essential devscripts ubuntu-standard software-properties-common \
        screen lxc traceroute gdb \
        vim git subversion mercurial cmake make \
        dos2unix nkf curl xz-utils graphviz imagemagick \
        openssh-server sudo autoconf automake libtool make unzip net-tools && \
    apt-get clean && rm -rf /var/lib/apt/lists/*
RUN mkdir -p /var/run/sshd

# Install protobuf.
RUN apt-get update -q && apt-get install -qy \
        libprotobuf-dev libprotoc-dev protobuf-compiler && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Set locale to suppress an sshd warning.
RUN echo 'LANG="en_US.UTF-8"' > /etc/default/locale

# Install additional packages.
RUN apt-get update && apt-get install -y libssl-dev && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Install gcloud.
RUN echo "deb http://packages.cloud.google.com/apt" \
        "cloud-sdk-$(lsb_release -c -s) main" \
        > /etc/apt/sources.list.d/google-cloud-sdk.list && \
    curl https://packages.cloud.google.com/apt/doc/apt-key.gpg | \
        apt-key add - && \
    apt-get update -qqy && \
    apt-get install -qqy google-cloud-sdk google-cloud-sdk-app-engine-go \
        google-cloud-sdk-app-engine-python google-cloud-sdk-app-engine-go \
        google-cloud-sdk-datastore-emulator && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Install Docker.
RUN curl -fsSL https://download.docker.com/linux/ubuntu/gpg | \
        apt-key add - && \
    add-apt-repository "$(echo \
        "deb [arch=amd64] https://download.docker.com/linux/ubuntu" \
        "$(lsb_release -cs)" stable)" && \
    apt-get update -qqy && \
    apt-get install -qyy docker-ce docker-ce-cli containerd.io && \
    curl -L https://github.com/docker/compose/releases/download/1.24.0/docker-compose-`uname -s`-`uname -m` \
        -o /usr/local/bin/docker-compose && \
    chmod +x /usr/local/bin/docker-compose && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

################################################################################
# Configurations
################################################################################

# Gcloud service account.
ADD ./service_account.json /root/.config/service_account.json
RUN gcloud auth activate-service-account \
    docker@icfpc-primary.iam.gserviceaccount.com \
    --key-file=/root/.config/service_account.json && \
    gcloud config set project icfpc-primary && \
    gcloud config set compute/region asia-northeast1 && \
    gcloud config set compute/zone asia-northeast1-a

# Create unagi user.
RUN useradd \
        --home-dir=/home/unagi \
        --create-home \
        --uid=10001 \
        --user-group \
        --shell=/bin/bash \
        unagi
RUN echo 'unagi ALL=(ALL:ALL) NOPASSWD: ALL' > /etc/sudoers.d/unagi

# Unagi password.
RUN echo "export UNAGI_PASSWORD='${UNAGI_PASSWORD}'" > /etc/profile.d/99-unagi.sh
RUN chmod +x /etc/profile.d/99-unagi.sh

# Docker configuration.
ADD ./docker_config.json /root/.docker/config.json

# Git settings.
RUN git config --global user.email '5896564+ninetan@users.noreply.github.com' && \
    git config --global user.name 'Ninetan'

# SSH settings.
ADD ./unagi.pem /root/.ssh/id_rsa
ADD ./unagi.pem /home/unagi/.ssh/id_rsa
RUN chmod 600 /root/.ssh/id_rsa /home/unagi/.ssh/id_rsa
ADD ./unagi.pub /root/.ssh/authorized_keys
ADD ./unagi.pub /home/unagi/.ssh/authorized_keys
RUN ssh-keyscan github.com >> /root/.ssh/known_hosts
RUN ssh-keyscan github.com >> /home/unagi/.ssh/known_hosts
RUN chown -R unagi:unagi /home/unagi/.ssh

# Add unagi command as proxy.
RUN echo '#!/usr/bin/env bash' > /usr/local/bin/unagi && \
    echo 'exec "$@"' >> /usr/local/bin/unagi && \
    chmod +x /usr/local/bin/unagi

# Mark as UNAGI_IMAGE.
RUN touch /UNAGI_IMAGE

################################################################################
# Experimental
################################################################################

RUN apt-get update && apt-get install -y jq && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

################################################################################
# Repository pull
################################################################################

# Download repository.
RUN git clone git@github.com:imos/icfpc2019.git /repo

# Fill quick survey.
RUN echo "last_answer_survey_time: $(date '+%s')" > \
    /root/.config/gcloud/.last_survey_prompt.yaml
