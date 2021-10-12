FROM rust:latest AS builder

WORKDIR /app

COPY src ./src
COPY Cargo.toml .

RUN cargo build --release

FROM debian:latest

COPY --from=builder /app/target/release/team-service .

EXPOSE 3030
ENTRYPOINT [ "./team-service" ]