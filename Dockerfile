# Zero to Prod book page 144 contains more information to slim down image size
FROM lukemathwalker/cargo-chef:latest-rust-1.77.1 AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef AS planner
COPY . .
# Compute a lock file
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE TRUE
RUN cargo build --release --bin zero2prod

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
	# OpenSSL is dynamically linked for some dependencies and CA Certs is needed to verify TLS certs during HTTPS connection establishment
	&& apt-get install -y --no-install-recommends openssl ca-certificates \
	# Clean up
	&& apt-get autoremove -y \
	&& apt-get clean -y \
	&& rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]
