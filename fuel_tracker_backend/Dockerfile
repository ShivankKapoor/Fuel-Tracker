# Use a Rust base image with the latest stable version
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml file to the working directory
COPY Cargo.toml .

# Copy the source code to the working directory
COPY src src

# Install dependencies
RUN cargo build --release

# Expose the port your application will listen on
EXPOSE 8080

# Specify the command to run when the container starts
CMD ["cargo", "run", "--release"]