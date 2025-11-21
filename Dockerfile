# Etapa 1: Builder - Compilar la aplicación Rust
FROM rust:1.83-slim as builder

WORKDIR /app

# Instalar dependencias del sistema necesarias
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copiar manifiestos
COPY Cargo.toml ./

# Copiar código fuente
COPY src ./src

# Compilar en modo release
RUN cargo build --release

# Etapa 2: Runtime - Imagen final ligera
FROM debian:bookworm-slim

WORKDIR /app

# Instalar dependencias de runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copiar el binario compilado
COPY --from=builder /app/target/release/api_tareas /app/api_tareas

# Exponer el puerto
EXPOSE 3000

# Comando para ejecutar la aplicación
CMD ["./api_tareas"]
