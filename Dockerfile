FROM rust:1.34.2-slim-stretch AS builder

COPY . /sources
WORKDIR /sources
RUN cargo build --release
RUN chown nobody:nogroup /sources/target/release/mateometer


FROM debian:stretch-slim
COPY --from=builder /sources/target/release/mateometer /mateometer

USER nobody
EXPOSE 8080
ENTRYPOINT ["/mateometer"]
