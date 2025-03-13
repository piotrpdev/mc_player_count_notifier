# MC Player Count Notifier

Periodically check if player list for Minecraft server has changed and send that information to a webhook.

## Usage

```bash
MC_HOSTNAME=12.456.789.123 \
MC_PORT=24352 \
WEBHOOK_URL='https://discord.com/api/webhooks/ID/SECRET' \
CHECK_INTERVAL=10 \     # seconds
mc_player_count_notifier
```
