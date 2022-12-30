FROM rust:slim-bullseye

RUN apt-get update && apt-get install libssl-dev pkg-config -y

WORKDIR /pacbot
COPY . /pacbot

RUN cargo install --path /pacbot && cargo clean

ENTRYPOINT ["/usr/local/cargo/bin/pacbot"]
