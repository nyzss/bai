# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /bai

# Copy the project's Cargo.toml and Cargo.lock files to the container
COPY . .

# Build the project dependencies without the source code
RUN cargo build --release

# Expose port 3000 (the port your Actix application will listen on)
EXPOSE 3000

# Start the Actix application
CMD ["./target/release/bai"]