####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN apt-get update && apt-get install -y libasound2-dev tini


WORKDIR /app

COPY ./ .

RUN cargo build  --release

####################################################################################################
## Final image
####################################################################################################
FROM scratch

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/release/podcli ./

# Use an unprivileged user.
USER dockeruser

ENTRYPOINT ["tini", "--", "/app/podcli"]
CMD ["-h"]
