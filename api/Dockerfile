# stage 1: build
FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

# create empty workspace to cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY api/Cargo.toml api/
COPY core/Cargo.toml core/

RUN mkdir -p api/src core/src && \
    echo "fn main() {}" > api/src/main.rs && \
    echo "pub fn dummy() {}" > core/src/lib.rs

# build dependencies
# we install specifically the binary we want, which pulls in dependencies
RUN cargo build --release --bin energonsoftware --locked

# remove dummy source
RUN rm -rf api/src core/src

# copy real source
COPY core/src core/src
COPY api/src api/src

# build app (touch main.rs to ensure rebuild)
RUN touch api/src/main.rs && cargo install --path api/ --root .

# stage 2: create image
FROM alpine:latest

# install dependencies
RUN apk add --no-cache libgcc ca-certificates

RUN adduser -S -D appuser

WORKDIR /app

USER appuser

# copy config
COPY etc/ etc/
COPY share/ share/

# copy binary
COPY --from=builder /app/bin/energonsoftware bin/

EXPOSE 8000

ENTRYPOINT ["bin/energonsoftware", "--prod"]
