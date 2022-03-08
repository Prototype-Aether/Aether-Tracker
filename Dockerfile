FROM rust:latest

WORKDIR /usr/src/aether-tracker

COPY . .

RUN cargo install --path .

CMD ["server"]