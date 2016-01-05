FROM python:2.7
MAINTAINER Messense Lv <messense@icloud.com>

ENV RUST_VERSION=1.5.0
ENV RUST_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static

COPY . /app

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    git \
    libssl-dev && \
    curl -sO $RUST_DIST_SERVER/dist/rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz && \
    tar -xzf rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz && \
    ./rust-$RUST_VERSION-x86_64-unknown-linux-gnu/install.sh --without=rust-docs && \
    DEBIAN_FRONTEND=noninteractive apt-get remove --purge -y curl && \
    DEBIAN_FRONTEND=noninteractive apt-get autoremove -y && \
    rm -rf rust-$RUST_VERSION-x86_64-unknown-linux-gnu \
    rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz \
    /var/lib/apt/lists/* \
    /tmp/* \
    /var/tmp/* && \
    cd /app && cargo build --release && cp /app/target/release/badwolf-run /usr/bin/badwolf-run  && \
    /usr/local/lib/rustlib/uninstall.sh && \
    rm -rf /app
