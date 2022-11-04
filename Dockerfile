FROM rust

WORKDIR /pacbot
COPY . /pacbot

RUN cargo install --path /pacbot

ENTRYPOINT ["/usr/local/cargo/bin/pacbot"]
