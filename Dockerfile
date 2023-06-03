FROM rust:latest

WORKDIR /usr/src/ahb

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/ahb"]