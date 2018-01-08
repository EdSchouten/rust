FROM ubuntu:17.10

COPY scripts/cloudabi-toolchain.sh /tmp/
RUN /tmp/cloudabi-toolchain.sh i686-unknown-cloudabi

COPY scripts/sccache.sh /scripts/
RUN sh /scripts/sccache.sh

ENV TARGETS i686-unknown-cloudabi
ENV CARGO_TARGET_I686_UNKNOWN_CLOUDABI_RUNNER true
ENV RUST_CONFIGURE_ARGS --target=$TARGETS --disable-jemalloc
ENV SCRIPT python2.7 /checkout/x.py test --target $TARGETS
