###############################################################################
## Builder
###############################################################################
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl && \
    apt-get update && \
    apt-get install -y \
        pkg-config \
        librust-alsa-sys-dev \
        musl-tools \
        build-essential \
        cmake \
        musl-dev \
        libssl-dev \
        libasound2-dev \
        && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

ARG TARGET=x86_64-unknown-linux-musl
ENV RUST_MUSL_CROSS_TARGET=$TARGET

WORKDIR /app

COPY ./ .

RUN cargo build  --target x86_64-unknown-linux-musl --release

###############################################################################
## Final image
###############################################################################
FROM alpine

RUN apk add tini libressl-dev

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/release/podcli ./

# Use an unprivileged user.
USER dockeruser

ENTRYPOINT ["tini", "--", "/app/podcli"]
CMD ["-h"]
