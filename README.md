# toronto-guard
A Discord bot to moderate Mason

## Features

- Disconnect Mason: allows anyone to disconnect Mason from a voice channel
- Mute Mason: allows anyone to mute Mason for 5 seconds

By default, there is a cooldown between uses (see Configuring bellow) to let Mason actually function on Discord.

## Permissions

- The bot requires the GUILD_MEMBERS intent.
- The bot also requires the MUTE_MEMBERS and MOVE_MEMBERS permissions on the server it is run on.

## Configuring

The bot has two configuration sources:
- **TOML file:** The default configuration file is `toronto-guard.toml` in the current working directory, but this can be overridden with the `TG_CONFIG_FILE` environment variable.
- **Environment variables:** Prefixed with `TG_`, supporting nested structures using `_` as a separator (case-sensitive).

Supported settings:

| TOML Key                    | Environment Variable           | Description                                                                                           |
|-----------------------------|--------------------------------|-------------------------------------------------------------------------------------------------------|
| -                           | `TG_CONFIG_FILE`               | Overrides the default path for the configuration file.                                                |
| `bot.token`                 | `TG_bot_token`                 | Discord bot token (required).                                                                         |
| `mason.user_id`             | `TG_mason_user_id`             | The Discord user ID of Mason (required). Get with right click, Copy User ID.                          |
| `mason.user_cooldown_sec`   | `TG_mason_user_cooldown_sec`   | The cooldown between any use of a moderation command, tracked per user, per command. Defaults to 600. |
| `mason.global_cooldown_sec` | `TG_mason_global_cooldown_sec` | The cooldown between any use of a moderation command, tracked per command only. Defaults to 300.      |
| `log.filter`                | `TG_log_filter`                | Logging filter (default: `"toronto-guard=info"`, see [env_logger’s documentation for more info][1]).  |

### Example `toronto-guard.toml`

```toml
[bot]
token = "your-discord-bot-token"  # Required

[mason]
user_id = "123456789012345678"  # Required
user_cooldown_sec = 450
global_cooldown_sec = 150

[log]
filter = "toronto-guard=debug"
```

[1]: https://github.com/rust-cli/env_logger
