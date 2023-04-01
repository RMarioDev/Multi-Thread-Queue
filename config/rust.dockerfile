FROM rust:latest

RUN cargo install cargo-watch

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["myapp"]
