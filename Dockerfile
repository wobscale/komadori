FROM rustlang/rust:nightly as builder

COPY . /usr/src/komadori
WORKDIR /usr/src/komadori
RUN cargo build --release

FROM debian:latest

COPY --from=builder /usr/src/komadori/target/release/backend /backend
COPY --from=builder /usr/src/komadori/target/release/api /api

ENV ENVIRONMENT=prod
