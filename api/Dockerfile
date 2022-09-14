FROM rust:1-slim-bullseye AS build
ARG KRATOS_VERSION=0.10.1
ARG KRATOS_HASH=ed6103aa6f85f0fd9095de2401138bd7f1725eb82d2c33b9546ca589a2d3a487

RUN apt-get update && apt-get install -y --no-install-recommends \
  ca-certificates \
  wget \
  pkg-config \
  && rm -rf /var/lib/apt/lists/*

RUN wget https://github.com/ory/kratos/releases/download/v${KRATOS_VERSION}/kratos_${KRATOS_VERSION}-linux_sqlite_64bit.tar.gz && \
  echo "${KRATOS_HASH} kratos_${KRATOS_VERSION}-linux_sqlite_64bit.tar.gz" | sha256sum -c && \
  tar -xzf kratos_${KRATOS_VERSION}-linux_sqlite_64bit.tar.gz && \
  mv kratos /usr/local/bin/kratos

WORKDIR /usr/src/api
COPY . .

RUN cargo install --locked --bin api --path .

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
  openssl \
  supervisor \
  && rm -rf /var/lib/apt/lists/*

COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf
COPY --from=build /usr/local/cargo/bin/api /usr/local/bin/api
COPY --from=build /usr/local/bin/kratos /usr/local/bin/kratos

CMD ["/usr/bin/supervisord"]
