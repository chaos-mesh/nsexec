# syntax=docker/dockerfile:experimental

FROM debian:buster-slim

ENV DEBIAN_FRONTEND noninteractive

ARG HTTPS_PROXY
ARG HTTP_PROXY
ARG TARGET=x86_64-unknown-linux-gnu
ARG CC=gcc

ENV http_proxy $HTTP_PROXY
ENV https_proxy $HTTPS_PROXY

RUN apt-get update && apt-get install build-essential $CC curl git pkg-config -y && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly-2020-07-01 -y
ENV PATH "/root/.cargo/bin:${PATH}"

RUN rustup target add $TARGET

RUN if [ -n "$HTTP_PROXY" ]; then echo "[http]\n\
proxy = \"${HTTP_PROXY}\"\n\
"\
>> /root/.cargo/config ; fi

RUN echo "[target.aarch64-unknown-linux-gnu]\n\
linker = \"aarch64-linux-gnu-gcc\"\n\
"\
>> /root/.cargo/config

COPY . /nsexec-build

WORKDIR /nsexec-build

ENV RUSTFLAGS "-Z relro-level=full"
RUN --mount=type=cache,target=/nsexec-build/target \
    --mount=type=cache,target=/root/.cargo/registry \
    cargo build --release --all --target $TARGET

RUN --mount=type=cache,target=/nsexec-build/target \
    cp /nsexec-build/target/$TARGET/release/nsexec /nsexec

RUN --mount=type=cache,target=/nsexec-build/target \
    cp /nsexec-build/target/$TARGET/release/libnsenter.so /libnsenter.so