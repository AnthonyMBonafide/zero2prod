# Zero to Prod book page 144 contains more information to slim down image size
FROM rust:1.77.1 AS builder

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE TRUE
RUN cargo build --release

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
