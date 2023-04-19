# We use the latest Rust stable release as base image
# Builder stage
FROM lukemathwalker/cargo-chef:latest AS chef

# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not 
# exist already 
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . . 
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin zero2prod

# Runtime stage
FROM debian:bullseye-slim AS runtime
WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/zero2prod /usr/local/bin
# We need the config at runtime
COPY configuration configuration
ENV APP_ENVIRONMENT production
# When `docker run` is executed, launch the binary 
ENTRYPOINT ["/usr/local/bin/zero2prod"]
