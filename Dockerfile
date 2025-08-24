FROM rust:latest AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

ENV SQLX_OFFLINE=true
COPY .sqlx ./.sqlx
COPY src ./src
RUN touch ./src/main.rs
RUN cargo build --release
RUN strip ./target/release/rugo



FROM debian:bookworm-slim AS release
WORKDIR /app
COPY --from=builder /app/target/release/rugo .

EXPOSE 8080
CMD ["./rugo"]
