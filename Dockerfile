FROM rust:1.86-bullseye as builder

WORKDIR /usr/src/your-money-left-the-chat
COPY . .

# Build the example in release mode
RUN cargo build --release --example your_money_left_the_chat

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl-dev \
    libsqlite3-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary and database
COPY --from=builder /usr/src/your-money-left-the-chat/target/release/examples/your_money_left_the_chat /usr/local/bin/your-money-left-the-chat

# Set default command
CMD ["your-money-left-the-chat"]
