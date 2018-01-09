FROM ubuntu:17.10

COPY scripts/cloudabi-toolchain.sh /tmp/
RUN /tmp/cloudabi-toolchain.sh x86_64-unknown-cloudabi

COPY scripts/sccache.sh /scripts/
RUN sh /scripts/sccache.sh

ENV CARGO_TARGET_X86_64_UNKNOWN_CLOUDABI_RUNNER true
ENV RUST_CONFIGURE_ARGS --target=x86_64-unknown-cloudabi --disable-jemalloc
ENV SCRIPT python2.7 /checkout/x.py test --target x86_64-unknown-cloudabi
