FROM rust:1.77.1

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE TRUE
RUN cargo build --release
ENTRYPOINT ["./target/release/zero2prod"]
