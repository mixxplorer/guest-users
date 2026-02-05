FROM rust:1

RUN apt-get update
RUN apt-get install -y libsqlite3-dev dbus systemd libsqlite3-dev dpkg dpkg-dev liblzma-dev libclang-dev libpam-dev libnss3-dev sudo

ARG REMOTE_USER
ARG REMOTE_UID
ARG REMOTE_GID

RUN groupadd --gid $REMOTE_GID $REMOTE_USER \
    && useradd --uid $REMOTE_UID --gid $REMOTE_GID -m $REMOTE_USER -s /bin/bash

# Allow calling sudo in devcontainer (e.g. to install additional packages), enable if required
# RUN echo $REMOTE_USER ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$REMOTE_USER

USER $REMOTE_USER

RUN cargo install diesel_cli --no-default-features --features "sqlite"
RUN rustup component add rustfmt
RUN rustup component add clippy
RUN cargo install cargo-audit --locked --features=fix
RUN cargo install cargo-deb
