FROM rust:bookworm as builder

ARG GITHUB_TOKEN
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

RUN USER=root cargo new --bin onlytax-api
COPY ./ ./onlytax-api
WORKDIR /onlytax-api

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

COPY --from=builder /onlytax-api/target/release/onlytax-api ${APP}/onlytax-api

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata openssl libpq5 \
    && rm -rf /var/lib/apt/lists/*
    
RUN chown -R $APP_USER:$APP_USER ${APP}
USER $APP_USER
WORKDIR ${APP}

ENV ENV=production
CMD ["./onlytax-api"]
