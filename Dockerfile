# Stage 1: Build the application
FROM rust:latest as builder

WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to the build environment
COPY Cargo.toml Cargo.lock ./

# Copy the source code to the build environment
COPY src ./src

# Build the application
RUN cargo build --release


# Stage 2: Create a lightweight production image
FROM alpine:latest

WORKDIR /usr/src/app

# Copy the built binary from the previous stage
COPY --from=builder /usr/src/app/target/release/rcp ./

# Set the environment variable for logging
ENV LOGGING_ENABLED true

# Expose the desired port
EXPOSE 8080

# Start the application
CMD ["./rcp"]