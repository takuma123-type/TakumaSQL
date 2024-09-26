FROM rust:1.67 as builder

WORKDIR /app
COPY . .  

RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app

COPY --from=builder /app/target/release/takumasql /app/takumasql

CMD ["./takumasql"]
