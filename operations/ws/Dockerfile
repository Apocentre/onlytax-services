FROM rust:bookworm as builder

ARG GITHUB_TOKEN
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

RUN USER=root cargo new --bin onlytax-ws-dist
COPY ./ ./onlytax-ws-dist
WORKDIR /onlytax-ws-dist

RUN apt-get update \
    && apt-get install -y protobuf-compiler libclang-dev clang \
    && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

FROM debian:bookworm-slim
ARG APP=/usr/src/app

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
  && useradd -g $APP_USER $APP_USER \
  && mkdir -p ${APP}

COPY --from=builder /onlytax-ws-dist/target/release/onlytax-ws ${APP}/onlytax-ws

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata openssl libpq5 \
    && rm -rf /var/lib/apt/lists/*
    
RUN chown -R $APP_USER:$APP_USER ${APP}
USER $APP_USER
WORKDIR ${APP}

ENV ENV=production
CMD ["./onlytax-ws"]
