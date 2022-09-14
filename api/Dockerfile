FROM rust:1-slim-bullseye AS build

RUN apt-get update && apt-get install -y --no-install-recommends \
  pkg-config \
  && rm -rf /var/lib/apt/lists/*

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

CMD ["/usr/bin/supervisord"]
