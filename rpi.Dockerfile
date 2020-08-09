## Building Stage ##
FROM messense/rust-musl-cross:armv7-musleabihf as builder
RUN rustup default nightly
RUN rustup target add armv7-unknown-linux-musleabihf
WORKDIR /build
# We'll get to what this file is below!
RUN echo "fn main() {}" > dummy.rs
# If this changed likely the Cargo.toml changed so lets trigger the
# recopying of it anyways
COPY Cargo.lock .
COPY Cargo.toml .
# We'll get to what this substitution is for but replace main.rs with
# lib.rs if this is a library
RUN sed -i 's%src/main.rs%dummy.rs%' Cargo.toml
# Drop release if you want debug builds. This step cache's our deps!
RUN cargo build --release --target armv7-unknown-linux-musleabihf
# Now return the file back to normal
RUN sed -i 's%dummy.rs%src/main.rs%' Cargo.toml
ADD . .
RUN cargo build --release --target armv7-unknown-linux-musleabihf

## Running stage ##
FROM arm32v7/alpine:3
EXPOSE 4000
COPY --from=builder /build/target/armv7-unknown-linux-musleabihf/release/potatosync-notes /usr/local/bin
ENV ROCKET_PORT=4000
CMD /usr/local/bin/potatosync-notes
