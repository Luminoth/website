# stage 1: build
FROM rust AS builder

WORKDIR /app

# copy source
COPY core/ core/
COPY api/ api/

# build
RUN cargo install --path api/ --root .

# stage 2: create image
FROM ubuntu

# install dependencies
RUN apt update && \
    apt install -y --no-install-recommends \
    ca-certificates

RUN useradd --create-home appuser

WORKDIR /app

USER appuser

# copy config
COPY etc/ etc/
COPY share/ share/

# copy binary
COPY --from=builder /app/bin bin/

EXPOSE 8000

CMD ["bin/energonsoftware"]
