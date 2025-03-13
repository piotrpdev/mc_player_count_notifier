# MC Player Count Notifier

Periodically check if player list for Minecraft server has changed and send
that information to a webhook.

## Usage

```bash
MC_HOSTNAME=12.456.789.123 \
MC_PORT=12345 \
WEBHOOK_URL='https://discord.com/api/webhooks/ID/SECRET' \
CHECK_INTERVAL=10 \     # seconds
mc_player_count_notifier
```

## Cross-compile for AArch64 musl

```bash
sudo apt install musl-tools gcc-aarch64-linux-gnu
rustup target add aarch64-unknown-linux-musl

CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc \
CC=aarch64-linux-gnu-gcc \
cargo build --release --target=aarch64-unknown-linux-musl
```
