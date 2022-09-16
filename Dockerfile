FROM rust:1.63-alpine

RUN apk upgrade --update-cache --available && \
    apk add openssl protoc musl-dev pkgconfig openssl-dev openldap-dev && \
    rm -rf /var/cache/apk/*
RUN cargo install cargo-shuttle

# RUN cargo shuttle login --api-key WHaKWE1GKRvDux3D
# COPY Cargo.toml Cargo.lock src ./
# RUN cargo shuttle deploy
#
