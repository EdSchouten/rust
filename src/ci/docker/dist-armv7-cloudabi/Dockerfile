FROM ubuntu:17.10

COPY scripts/cloudabi-toolchain.sh /tmp/
RUN /tmp/cloudabi-toolchain.sh armv7-unknown-cloudabi-eabihf

COPY scripts/sccache.sh /scripts/
RUN sh /scripts/sccache.sh

ENV CARGO_TARGET_ARMV7_UNKNOWN_CLOUDABI_EABIHF_RUNNER true
ENV RUST_CONFIGURE_ARGS --target=armv7-unknown-cloudabi --disable-jemalloc-eabihf
ENV SCRIPT python2.7 /checkout/x.py test --target armv7-unknown-cloudabi-eabihf
