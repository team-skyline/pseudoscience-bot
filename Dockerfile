FROM rust:1 as builder

WORKDIR /src
COPY . .

RUN cargo install --path .

FROM debian:bookworm-slim

ARG GH_VERSION=2.40.1
ARG ARCH=amd64

RUN apt update \
    && apt install -y git grep sed wget \
    && rm -rf /var/lib/apt/lists/*

RUN wget https://github.com/cli/cli/releases/download/v${GH_VERSION}/gh_${GH_VERSION}_linux_${ARCH}.deb \
    && apt install -y ./gh_${GH_VERSION}_linux_${ARCH}.deb \
    && rm ./gh_${GH_VERSION}_linux_${ARCH}.deb

RUN useradd -m -u 1000 -s /bin/sh appuser \
    && mkdir -p /data \
    && chown -R appuser /data

COPY --from=builder /usr/local/cargo/bin/pseudoscience-bot /usr/local/bin/pseudoscience-bot

USER appuser

ENV RUST_LOG="info"
ENV PACKWIZ_REPO_PATH="/data"
ENV DB_PATH="/db"

VOLUME ["/data"]
VOLUME ["/db"]
VOLUME ["/home/appuser"]

CMD ["pseudoscience-bot"]