FROM messense/badwolf-test-runner:base
MAINTAINER Messense Lv <messense@icloud.com>

ENV RUST_VERSION=1.7.0
ENV RUST_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static

RUN curl -sO $RUST_DIST_SERVER/dist/rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz && \
    tar -xzf rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz && \
    ./rust-$RUST_VERSION-x86_64-unknown-linux-gnu/install.sh --without=rust-docs && \
    rm -rf rust-$RUST_VERSION-x86_64-unknown-linux-gnu \
    rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz \
    /var/lib/apt/lists/* \
    /tmp/* \
    /var/tmp/*
