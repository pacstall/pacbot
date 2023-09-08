FROM rust as build

WORKDIR /pacbot
COPY . /pacbot

RUN RUSTFLAGS='-C target-feature=+crt-static' cargo install --locked --path /pacbot --target x86_64-unknown-linux-gnu && cargo clean

FROM gcr.io/distroless/cc

WORKDIR /pacbot
COPY --from=build /usr/local/cargo/bin/pacbot /pacbot/

ENTRYPOINT ["/pacbot/pacbot"]
