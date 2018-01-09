FROM ubuntu:17.10

ENV TARGETS=aarch64-unknown-cloudabi
# TODO(ed): Enable this once a new libc has been released.
# ENV TARGETS=armv7-unknown-cloudabi-eabihf
ENV TARGETS=$TARGETS,i686-unknown-cloudabi
ENV TARGETS=$TARGETS,x86_64-unknown-cloudabi

COPY scripts/cloudabi-toolchain.sh /tmp/
RUN /tmp/cloudabi-toolchain.sh

COPY scripts/sccache.sh /scripts/
RUN sh /scripts/sccache.sh

ENV CARGO_TARGET_AARCH64_UNKNOWN_CLOUDABI_RUNNER true
ENV CARGO_TARGET_I686_UNKNOWN_CLOUDABI_RUNNER true
ENV CARGO_TARGET_X86_64_UNKNOWN_CLOUDABI_RUNNER true

ENV RUST_CONFIGURE_ARGS --target=${TARGETS} --disable-jemalloc
ENV SCRIPT \
        python2.7 /checkout/x.py test --target ${TARGETS} && \
        python2.7 /checkout/x.py dist --target ${TARGETS}
