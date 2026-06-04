###############################################################################
## Builder
###############################################################################
FROM rust:latest AS builder
LABEL maintainer="Lorenzo Carbonell <a.k.a. atareao> lorenzo.carbonell.cerezo@gmail.com"
ARG TARGETPLATFORM

ENV RUST_MUSL_CROSS_TARGET=$TARGETPLATFORM

COPY ./platform.sh /platform.sh
RUN /platform.sh && \
    echo $TARGETPLATFORM && \
    cat /.target

RUN rustup target add "$(cat /.target)" && \
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
RUN cross build --release --target $(cat /.target) && \
    cp /app/target/$(cat /.target)/release/podcli /app/podcli

###############################################################################
## Final image
###############################################################################
FROM alpine

RUN apk add tini libressl-dev

WORKDIR /app

# Copy our build
COPY --from=builder /app/podcli ./

# Use an unprivileged user.
USER dockeruser

CMD ["/app/podcli"]
