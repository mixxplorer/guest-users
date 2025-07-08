FROM rust:1

RUN apt-get update
RUN apt-get install -y libsqlite3-dev dbus systemd libsqlite3-dev dpkg dpkg-dev liblzma-dev libclang-dev libpam-dev libnss3-dev
RUN cargo install diesel_cli --no-default-features --features "sqlite"
RUN rustup component add rustfmt
RUN rustup component add clippy
RUN cargo install cargo-audit --locked --features=fix
RUN cargo install cargo-deb
