FROM rust:1.75-bullseye as builder

WORKDIR /usr/src/ingress-dns
COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libc6 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/ingress-dns /usr/local/bin/ingress-dns
CMD ["ingress-dns"]