FROM rust:1.69
COPY . /distributed/

WORKDIR /distributed/backend
RUN cargo build --release --bin hotel

CMD ["/distributed/backend/target/release/hotel"]