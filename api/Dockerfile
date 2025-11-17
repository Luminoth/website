# stage 1: build
# TODO: make this use the local cargo cache
FROM rust:latest AS builder

WORKDIR /app

# copy source
COPY core/ core/
COPY api/ api/

# build
RUN cargo install --path api/ --root .

# stage 2: create image
# TODO: find a way to move this to alpine:latest
#FROM debian:stable-slim (this is larger than using ubuntu latest ???)
FROM ubuntu:latest

# install dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --create-home appuser

WORKDIR /app

USER appuser

# copy config
COPY etc/ etc/
COPY share/ share/

# copy binary
COPY --from=builder /app/bin bin/

EXPOSE 8000

ENTRYPOINT ["bin/energonsoftware", "--prod"]
