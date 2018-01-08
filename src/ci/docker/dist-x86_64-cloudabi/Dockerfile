FROM ubuntu:17.10

RUN apt-get update && apt-get install -y --no-install-recommends \
  apt-transport-https \
  ca-certificates \
  clang-5.0 \
  cmake \
  curl \
  file \
  g++ \
  gdb \
  git \
  lld-5.0 \
  make \
  python \
  sudo \
  xz-utils

COPY scripts/cloudabi-toolchain.sh /tmp/
RUN /tmp/cloudabi-toolchain.sh x86_64-unknown-cloudabi

COPY scripts/sccache.sh /scripts/
RUN sh /scripts/sccache.sh

ENV TARGETS=x86_64-unknown-cloudabi

ENV CARGO_TARGET_X86_64_UNKNOWN_CLOUDABI_RUNNER true
ENV RUST_CONFIGURE_ARGS --target=$TARGETS --disable-jemalloc

ENV SCRIPT python2.7 /checkout/x.py test --target $TARGETS
