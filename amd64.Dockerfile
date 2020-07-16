## Building Stage ##
FROM messense/rust-musl-cross:x86_64-musl as builder
RUN rustup default nightly
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /build
ADD . .
RUN cargo build --release --target x86_64-unknown-linux-musl


## Running stage ##
FROM amd64/alpine:3
EXPOSE 4000
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/potatosync-notes /usr/local/bin
ENV ROCKET_PORT=4000
CMD /usr/local/bin/potatosync-notes
