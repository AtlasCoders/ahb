FROM rust:latest
WORKDIR /usr/src/ahb
COPY Cargo.toml .
COPY src ./src
RUN cargo install --path .
CMD ["ahb", "--help"]
