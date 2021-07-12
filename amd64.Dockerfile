## Building Stage ##
FROM rust:1.53 as builder

# Build time options to avoid dpkg warnings and help with reproducible builds.
ENV DEBIAN_FRONTEND=noninteractive LANG=C.UTF-8 TZ=UTC TERM=xterm-256color

# Install DB packages
RUN apt-get update && apt-get install -y \
    --no-install-recommends \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# We'll get to what this file is below!
RUN mkdir src
RUN echo "fn main() {}" > dummy.rs

# If this changed likely the Cargo.toml changed so lets trigger the
# recopying of it anyways
COPY Cargo.lock .
COPY Cargo.toml .

# We'll get to what this substitution is for but replace main.rs with
# lib.rs if this is a library
RUN sed -i 's%src/main.rs%dummy.rs%' Cargo.toml

# Drop release if you want debug builds. This step cache's our deps!
RUN cargo build --release
# Now return the file back to normal
RUN sed -i 's%dummy.rs%src/main.rs%' Cargo.toml
ADD . .
RUN cargo build --release


## Running stage ##
FROM amd64/alpine:3
EXPOSE 4000
RUN apk add libpq
COPY --from=builder /build/target/release/potatosync-notes /usr/local/bin
CMD /usr/local/bin/potatosync-notes
