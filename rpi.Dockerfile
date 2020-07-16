## Building Stage ##
FROM messense/rust-musl-cross:armv7-musleabihf as builder
RUN rustup default nightly
RUN rustup target add armv7-unknown-linux-musleabihf
WORKDIR /build
ADD . .
RUN cargo build --release --target armv7-unknown-linux-musleabihf


## Running stage ##
FROM arm32v7/alpine:3
EXPOSE 4000
COPY --from=builder /build/target/armv7-unknown-linux-musleabihf/release/potatosync-notes /usr/local/bin
ENV ROCKET_PORT=4000
CMD /usr/local/bin/potatosync-notes
