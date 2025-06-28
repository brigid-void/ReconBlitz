FROM rust:1.70-slim-bullseye as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/reconblitz /usr/local/bin

ENTRYPOINT ["reconblitz"]
