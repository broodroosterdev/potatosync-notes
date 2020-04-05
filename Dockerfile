# -*- mode: dockerfile -*-
#
# An example Dockerfile showing how to build a Rust executable using this
# image, and deploy it with a tiny Alpine Linux container.

# You can override this `--build-arg BASE_IMAGE=...` to use different
# version of Rust or OpenSSL.
ARG BASE_IMAGE=registry.gitlab.com/rust_musl_docker/image:nightly-2020-04-02

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder

# Add our source code.
ADD . /workdir
# Build our application.
RUN cargo build --release -vv --target=x86_64-unknown-linux-musl

# Now, we need to build our _real_ Docker container, copying in `using-diesel`.
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /workdir/target/x86_64-unknown-linux-musl/release/potatosync-rust \
    /usr/local/bin/
WORKDIR /usr/local/bin
ENV ROCKET_PORT=4000
CMD /usr/local/bin/potatosync-rust