# Build stage for the Rust backend
FROM rust:1.79-bullseye AS rust-builder

WORKDIR /app

ARG DATABASE_URL

ENV DATABASE_URL=$DATABASE_URL

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /user/local/bin

COPY --from=rust-builder /app/target/release/backend .

CMD [ "./backend" ]

