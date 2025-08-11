FROM rust:1.88-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/discord_reaction_info_selenity /usr/local/bin/
CMD ["discord_reaction_info_selenity"]