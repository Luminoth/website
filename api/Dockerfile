# stage 1: build
FROM rust AS builder

# update image
RUN apt update && apt -y upgrade

WORKDIR /app

# copy source
COPY core/ core/
COPY api/ api/

# build
RUN cargo install --path api/ --root .

# stage 2: create image
FROM ubuntu

# update image
RUN apt update && apt -y upgrade

# install dependencies
RUN apt install -y ca-certificates

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
