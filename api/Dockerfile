FROM rust:1-alpine3.16 AS build
ARG KRATOS_VERSION=0.10.1

WORKDIR /usr/src/api
COPY . .

RUN cargo install --locked --bin api --path .
RUN wget https://github.com/ory/kratos/releases/download/v${KRATOS_VERSION}/kratos_${KRATOS_VERSION}-linux_sqlite_64bit.tar.gz && \
  tar -xzf kratos_${KRATOS_VERSION}-linux_sqlite_64bit.tar.gz && \
  mv kratos /usr/local/bin/kratos

FROM alpine:3.16

RUN apk add --no-cache supervisor

COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf
COPY --from=builder /usr/local/cargo/bin/api /usr/local/bin/api
COPY --from=builder /usr/local/bin/kratos /usr/local/bin/kratos

CMD ["/usr/bin/supervisord"]
