# syntax=docker/dockerfile:experimental

FROM debian:buster-slim

ENV DEBIAN_FRONTEND noninteractive

ARG HTTPS_PROXY
ARG HTTP_PROXY

ENV http_proxy $HTTP_PROXY
ENV https_proxy $HTTPS_PROXY

RUN apt-get update && apt-get install build-essential curl git pkg-config -y && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly-2020-07-01 -y
ENV PATH "/root/.cargo/bin:${PATH}"

RUN if [ -n "$HTTP_PROXY" ]; then echo "[http]\n\
proxy = \"${HTTP_PROXY}\"\n\
"\
> /root/.cargo/config ; fi

COPY . /nsexec-build

WORKDIR /nsexec-build

ENV RUSTFLAGS "-Z relro-level=full"
RUN --mount=type=cache,target=/nsexec-build/target \
    --mount=type=cache,target=/root/.cargo/registry \
    cargo build --release --all

RUN --mount=type=cache,target=/nsexec-build/target \
    cp /nsexec-build/target/release/nsexec /nsexec

RUN --mount=type=cache,target=/nsexec-build/target \
    cp /nsexec-build/target/release/libnsenter.so /libnsenter.so