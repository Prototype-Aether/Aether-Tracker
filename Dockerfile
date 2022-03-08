FROM rust:1.58

WORKDIR /usr/src/aether-tracker

COPY . .

RUN cargo install --path .

CMD ["server"]