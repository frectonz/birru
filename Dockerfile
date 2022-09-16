FROM rust

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/birru"]
