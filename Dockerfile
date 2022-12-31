FROM rust as build

WORKDIR /pacbot
COPY . /pacbot

RUN cargo install --path /pacbot && cargo clean

FROM gcr.io/distroless/cc

WORKDIR /pacbot
COPY --from=build /usr/local/cargo/bin/pacbot /pacbot/

ENTRYPOINT ["/pacbot/pacbot"]
