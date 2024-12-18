# Use Rust official image
FROM rust:1.72-slim

# # Install cargo-watch
# RUN cargo install cargo-watch

# Set the working directory
WORKDIR /usr/src/app

# Copy Cargo.toml and src code
COPY Cargo.toml .
COPY src ./src

# Copy the CSV file into the image
COPY data.csv ./data.csv

# Build the project
RUN cargo build --release

# Expose the port for the API
EXPOSE 8080

# Set the startup command
CMD ["./target/release/rust-api"]
