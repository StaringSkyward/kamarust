# build stage
FROM rust:1.83-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# run stage
FROM rust:1.83-slim-bookworm
WORKDIR /app
COPY --from=builder /app/target/release/kamarust /app/kamarust
EXPOSE 80
CMD ["./kamarust"]
