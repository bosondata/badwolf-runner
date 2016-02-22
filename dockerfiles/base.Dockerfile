FROM ubuntu:trusty
MAINTAINER Messense Lv <messense@icloud.com>

ENV RUST_VERSION=1.6.0
ENV RUST_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
# ENV RUST_DIST_SERVER=https://static.rust-lang.org

COPY . /app

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    wget \
    git \
    libssl-dev && \
    curl -sO $RUST_DIST_SERVER/dist/rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz && \
    tar -xzf rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz && \
    ./rust-$RUST_VERSION-x86_64-unknown-linux-gnu/install.sh --without=rust-docs && \
    DEBIAN_FRONTEND=noninteractive apt-get autoremove -y && \
    rm -rf rust-$RUST_VERSION-x86_64-unknown-linux-gnu \
    rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz \
    /var/lib/apt/lists/* \
    /tmp/* \
    /var/tmp/* && \
    mkdir -p ~/.cargo && \
    cp /app/conf/cargo-config ~/.cargo/config && \
    cd /app && cargo build --release && cargo test && cp /app/target/release/badwolf-run /usr/bin/badwolf-run  && \
    /usr/local/lib/rustlib/uninstall.sh && \
    rm -rf /app \
    ~/.cargo
