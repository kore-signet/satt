FROM node:20-bullseye-slim as nodebuilder

WORKDIR /usr/src/
COPY frontend frontend
WORKDIR /usr/src/frontend
RUN npm install .
RUN npm run build


FROM rust:1.70.0-buster as builder

WORKDIR /usr/src/joie

COPY Cargo.toml . 
COPY src src

RUN cargo build --profile production --bin satt

FROM debian:buster-slim

RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install wget -y

WORKDIR /joie
RUN chown -R 1000:1000 /joie

USER 1000

RUN mkdir run
WORKDIR run
COPY --from=builder /usr/src/joie/target/production/satt .
COPY --from=nodebuilder /usr/src/frontend/dist static

CMD ["./satt"]