# Build stage
FROM rust:latest AS builder

RUN update-ca-certificates

# Create unprivileged app user
ENV USER=kamarust
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /app

COPY . .

RUN cargo build --release

# Final image
FROM debian:bookworm-slim

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

COPY --from=builder /app/target/release/kamarust ./

EXPOSE 80

# Use an unprivileged user.
USER kamarust:kamarust

CMD ["/app/kamarust"]
