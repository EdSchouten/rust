FROM ubuntu:17.10

RUN apt-get update && apt-get install -y --no-install-recommends \
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

RUN ln -s llvm-ar-5.0 /usr/bin/x86_64-unknown-cloudabi-ar && \
    ln -s clang-5.0 /usr/bin/x86_64-unknown-cloudabi-cc && \
    ln -s clang++-5.0 /usr/bin/x86_64-unknown-cloudabi-c++ && \
    ln -s lld-5.0 /usr/bin/x86_64-unknown-cloudabi-ld

COPY scripts/sccache.sh /scripts/
RUN sh /scripts/sccache.sh

# TODO(ed): Remove this once cc-rs has been released?
ENV \
    AR_x86_64_unknown_cloudabi=x86_64-unknown-cloudabi-ar \
    CC_x86_64_unknown_cloudabi=x86_64-unknown-cloudabi-cc \
    CXX_x86_64_unknown_cloudabi=x86_64-unknown-cloudabi-c++

ENV TARGETS=x86_64-unknown-cloudabi

ENV RUST_CONFIGURE_ARGS --target=$TARGETS

ENV SCRIPT python2.7 ../x.py test --target $TARGETS
