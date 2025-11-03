# Stage 1: Build
FROM rust
WORKDIR /usr/src/app

# Copy manifests first to cache dependencies
COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

# Copy the source 
COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/myapp"]
