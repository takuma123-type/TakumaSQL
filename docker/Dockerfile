FROM rust:1.67 as builder

WORKDIR /app

# Cargo.toml のみをコピー
COPY Cargo.toml ./

# 依存関係を事前に解決
RUN cargo fetch

# 残りのソースコードをコピー
COPY . .

# 必要な場合は依存関係をインストール
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# リリースビルドを実行
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app

# ビルドしたバイナリをコピー
COPY --from=builder /app/target/release/takumasql /app/takumasql

CMD ["./takumasql"]
