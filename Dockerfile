# We use the latest Rust stable release as base image 
FROM rust:1.63.0-alpine

# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not 
# exist already 
WORKDIR /app 
# Install the required system dependencies for our linking configuration
RUN apk update && \
  apk add \
  clang \
  lld \
  build-base \
  libressl-dev
# Copy all files from our working environment to our Docker image 
COPY . . 
# Let's build our binary
# We'll use the release profile to make it fast 
ENV SQLX_OFFLINE true
RUN cargo build --release
ENV APP_ENVIRONMENT production
# When `docker run` is executed, launch the binary 
ENTRYPOINT ["./target/release/zero2prod"]