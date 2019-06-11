FROM rust:1.34.2-slim-stretch AS builder

COPY . /sources
WORKDIR /sources
RUN rustup override set nightly
RUN cargo build --release
RUN chown nobody:nogroup /sources/target/release/mateometer


FROM debian:stretch-slim

COPY --from=builder /sources/target/release/mateometer /mateometer
ENTRYPOINT ["/mateometer"]

USER nobody
EXPOSE 8000
ENV ROCKET_ENV production

