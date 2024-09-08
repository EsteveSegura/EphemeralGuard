FROM rust:latest AS builder

WORKDIR /usr/src/app
COPY . .

RUN apt-get update && apt-get install -y pkg-config
RUN cargo build --release

FROM ubuntu:latest

RUN apt-get update && apt-get install -y pkg-config
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/ephemeral_guard .

EXPOSE 1337
CMD ["./ephemeral_guard"]
