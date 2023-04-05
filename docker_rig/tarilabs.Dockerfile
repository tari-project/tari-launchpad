# syntax = docker/dockerfile:1.3

# https://hub.docker.com/_/rust
ARG RUST_VERSION=1.68

# rust source compile with cross platform build support
FROM --platform=$BUILDPLATFORM rust:$RUST_VERSION-bullseye as builder

# Declare to make available
ARG BUILDPLATFORM
ARG BUILDOS
ARG BUILDARCH
ARG BUILDVARIANT
ARG TARGETPLATFORM
ARG TARGETOS
ARG TARGETARCH
ARG TARGETVARIANT
ARG RUST_TOOLCHAIN
ARG RUST_TARGET
ARG RUST_VERSION

RUN apt-get update && apt-get install -y \
  apt-transport-https \
  bash \
  ca-certificates \
  curl \
  gpg \
  iputils-ping \
  less \
  libreadline-dev \
  libsqlite3-0 \
  openssl \
  telnet \
  cargo \
  clang \
  gcc-aarch64-linux-gnu \
  g++-aarch64-linux-gnu \
  cmake

ARG ARCH=native
#ARG FEATURES=avx2
ARG FEATURES=safe
ENV RUSTFLAGS="-C target_cpu=$ARCH"
ENV ROARING_ARCH=$ARCH
ENV CARGO_HTTP_MULTIPLEXING=false

ARG VERSION=1.0.1
ARG APP_NAME=wallet
ARG APP_EXEC=tari_console_wallet
ARG TARI_NETWORK

RUN if [ "${BUILDARCH}" != "${TARGETARCH}" ] && [ "${ARCH}" = "native" ] ; then \
      echo "!! Cross-compile and native ARCH not a good idea !! " ; \
    fi

WORKDIR /tari

ADD Cargo.toml .
ADD Cargo.lock .
ADD rust-toolchain.toml .
ADD applications applications
ADD base_layer base_layer
ADD clients clients
ADD common common
ADD common_sqlite common_sqlite
ADD comms comms
ADD infrastructure infrastructure
ADD meta meta
ADD buildtools/deps_only buildtools/deps_only
ADD integration_tests integration_tests

RUN if [ "${TARGETARCH}" = "arm64" ] && [ "${BUILDARCH}" != "${TARGETARCH}" ] ; then \
      # Hardcoded ARM64 envs for cross-compiling - FixMe soon
      export BUILD_TARGET="aarch64-unknown-linux-gnu/" && \
      export RUST_TARGET="--target=aarch64-unknown-linux-gnu" && \
      export ARCH=generic && \
      export FEATURES=safe && \
      export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc && \
      export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc && \
      export CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++ && \
      export BINDGEN_EXTRA_CLANG_ARGS="--sysroot /usr/aarch64-linux-gnu/include/" && \
      export RUSTFLAGS="-C target_cpu=$ARCH" && \
      export ROARING_ARCH=$ARCH && \
      rustup target add aarch64-unknown-linux-gnu && \
      rustup toolchain install stable-aarch64-unknown-linux-gnu --force-non-host ; \
    fi && \
    if [ -n "${RUST_TOOLCHAIN}" ] ; then \
      # Install a non-standard toolchain if it has been requested.
      # By default we use the toolchain specified in rust-toolchain.toml
      rustup toolchain install ${RUST_TOOLCHAIN} --force-non-host ; \
    fi && \
    rustup target list --installed && \
    rustup toolchain list && \
    cargo build ${RUST_TARGET} \
      --bin ${APP_EXEC} --release --features ${FEATURES} --locked && \
    # Copy executable out of the cache so it is available in the runtime image.
    cp -v /tari/target/${BUILD_TARGET}release/${APP_EXEC} /tari/${APP_EXEC}

# Create runtime base minimal image for the target platform executables
FROM --platform=$TARGETPLATFORM bitnami/minideb:bullseye as runtime

ARG BUILDPLATFORM
ARG TARGETOS
ARG TARGETARCH
ARG TARGETVARIANT
ARG RUST_VERSION

ARG VERSION

ARG APP_NAME
ARG APP_EXEC
ARG TARI_NETWORK

# Disable Prompt During Packages Installation
ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get --no-install-recommends install -y \
  apt-transport-https \
  bash \
  ca-certificates \
  curl \
  gpg \
  iputils-ping \
  less \
  libreadline8 \
  libreadline-dev \
  libsqlite3-0 \
  openssl \
  telnet

RUN groupadd --gid 1000 tari && \
    useradd --create-home --no-log-init --shell /bin/bash \
      --home-dir /var/tari \
      --uid 1000 --gid 1000 tari

ENV dockerfile_version=$VERSION
ENV dockerfile_build_arch=$BUILDPLATFORM
ENV rust_version=$RUST_VERSION
ENV APP_NAME=${APP_NAME:-wallet}
ENV APP_EXEC=${APP_EXEC:-tari_console_wallet}
ENV TARI_NETWORK=$TARI_NETWORK

RUN mkdir -p "/var/tari/${APP_NAME}" && \
    chown -R tari:tari "/var/tari/${APP_NAME}"

# Setup blockchain path for base_node only
RUN if [ "${APP_NAME}" = "base_node" ] ; then \
      echo "Base_node bits" && \
      mkdir /blockchain && \
      chown -R tari:tari /blockchain && \
      chmod -R 0700 /blockchain ; \
    else \
      echo "Not base_node" ; \
    fi

USER tari

COPY --from=builder /tari/$APP_EXEC /usr/local/bin/
COPY buildtools/docker_rig/start_tari_app.sh /usr/local/bin/start_tari_app.sh

ENTRYPOINT [ "start_tari_app.sh" ]
CMD [ "--non-interactive-mode" ]
