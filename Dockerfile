# Etap 1: Budowanie aplikacji
FROM rust:1.85 AS builder
WORKDIR /usr/src/app

# Skopiuj pliki Cargo i pobierz zależności (opcjonalne dla cache)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

# Skopiuj resztę plików źródłowych
COPY . .

# Zbuduj projekt w trybie release
RUN cargo build --release

# Etap 2: Utworzenie finalnego obrazu
FROM debian:buster-slim

# Instalacja certyfikatów (jeśli są potrzebne)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Skopiuj zbudowany binarny plik z etapu budowania
COPY --from=builder /usr/src/app/target/release/resource-monitor .

# Ustaw domyślne polecenie
CMD ["./resource-monitor"]
