FROM rust:1.83-alpine AS builder
WORKDIR /app
RUN apk add --no-cache musl-dev
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release

FROM alpine:3.20
WORKDIR /app
COPY --from=builder /app/target/release/stitchwork-server .
EXPOSE 3000
CMD ["./stitchwork-server"]
