FROM rust:latest

WORKDIR /app
COPY . .

RUN cargo build --release

EXPOSE 8000

CMD ["./target/release/my_rust_webserver"]