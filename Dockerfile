FROM rust:1.60 AS builder
WORKDIR app
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY ./src ./src
RUN cargo build --release

FROM debian:latest
ARG TOKEN
ENV DISCORD_TOKEN=${TOKEN}
COPY --from=builder /app/target/release/insider-bot-rs /usr/local/bin/insider-bot-rs
CMD ["/usr/local/bin/insider-bot-rs"]
