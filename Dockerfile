FROM rust:alpine AS build-stage

RUN apk update
RUN apk add cmake make musl-dev g++ perl

WORKDIR /build
COPY Cargo.toml ./
COPY src ./src
COPY test-files ./test-files
# Run tests before release build
RUN cargo test --release
RUN cargo build --release

# Build image from scratch
FROM scratch
LABEL org.opencontainers.image.source="https://github.com/diz-unimr/molgen-to-kafka"
LABEL org.opencontainers.image.licenses="AGPL-3.0-or-later"
LABEL org.opencontainers.image.description="Send MolGen-JSON to a Kafka broker"

COPY --from=build-stage /build/target/release/molgen-to-kafka .
USER 65532
EXPOSE 3000
CMD ["./molgen-to-kafka"]
