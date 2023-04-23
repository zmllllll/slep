# build a Rust using rust-musl-builder image,
# and deploy it with a tiny Alpine Linux container.

# You can override this `--build-arg BASE_IMAGE=...`
# to use different version of Rust or OpenSSL.
ARG BASE_IMAGE=49.232.216.15:8000/shengqian/rust-musl-builder:nightly-2022-12-20

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder

USER root

# Add our source code.
# ADD --chown=rust:rust . ./
ADD --chown=rust:rust ./core/src ./src
ADD --chown=rust:rust ./core/cargo_for_docker/build.rs ./build.rs
ADD --chown=rust:rust ./core/cargo_for_docker/Cargo.toml ./Cargo.toml
ADD --chown=rust:rust ./core/migrations ./migrations
ADD --chown=rust:rust ./json.core ./json.core
ADD --chown=rust:rust ./payload ./payload
ADD --chown=rust:rust ./resource ./resource
# ADD --chown=rust:rust ./tls ./tls
RUN git clone https://quakegit.cn/Qians/grpc-proto.git
RUN apt-get update && sudo apt-get upgrade -y
RUN apt-get install -y protobuf-compiler libprotobuf-dev

# ADD --chown=rust:rust ./core/grpc-proto ./grpc-proto

RUN ls

# RUN git clone https://quakegit.cn/Qians/grpc-proto.git
# Build our application.
RUN cargo build --release

# Now, we need to build our _real_ Docker container, copy to dest image.
FROM alpine:latest
RUN apk --no-cache add ca-certificates
WORKDIR /slep-core
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/slep-core \
    ./
EXPOSE 8080
CMD ["./slep-core", "0.0.0.0:8080"]