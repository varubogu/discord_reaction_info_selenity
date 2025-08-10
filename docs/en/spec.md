# Project Overview

## Overview

A Discord Bot that extracts reaction information from Discord messages.
When a command is entered, it collects the types of reactions and the people who reacted, then notifies users with a message.

Multi-language support is provided. Currently supports Japanese and English.

## Command Overview

### Reaction Members: Display information about users who reacted

Specify a message and collect reaction information from that message to post as a message to the user.
The posted message has the user part surrounded by code blocks.
The intention is to make it easy to copy and paste, and to avoid actual mentions.
The result message of this command is notified only to the user.

#### Constraints

- The user executing the command and the Bot must have read permissions for the message and reactions

#### Provided Forms

- ✕: Message event
- ◯: Slash command (command name: reaction_members)
- ◯: Message context menu (menu names: "Get reaction members", "Get reaction-grouping members")
- ✕: User context menu

#### Slash Command Syntax

```txt
/reaction_members message [is_author_include] [is_show_count] [is_reaction_grouping]
```

#### Slash Command Parameters

- message: Message (required)
    - Message URL or message ID
- is_author_include: bool (optional, default: false)
    - Whether to include the message sender in the results
- is_show_count: bool (optional, default: false)
    - Whether to include reaction count display
- is_reaction_grouping: bool (optional, default: false)
    - True: Aggregates users for each reaction
    - False: Aggregates users by combining all reactions

#### Slash Command Usage Examples

Message ID specification (basic)

```txt
/reaction_members message:1234567890
```

Message URL specification with count display

```txt
/reaction_members message:https://discord.com/channels/111111/222222/333333 is_show_count:True
```

Display grouped by reaction

```txt
/reaction_members message:1234567890 is_reaction_grouping:True
```

Display including message author

```txt
/reaction_members message:1234567890 is_author_include:True
```

#### Response Examples

Assuming the following message, examples for each parameter setting are described.

```
Message URL:
  https://discord.com/channels/{guild_id}/{channel_id}/{message_id}
Message sender:
  @user_a
Reactions:
  👍: @user_a @user_b
  ❤️: @user_c
  😂: @user_c @user_d
```

- Basic (no parameters)

```txt
Information
  📝: <Link to message>

Users:
  @user_a @user_b @user_c @user_d
```

- is_reaction_grouping=True (grouped by reaction)

```txt
Information
  📝: <Link to message>

Reactions:
  👍: @user_a @user_b
  ❤️: @user_c
  😂: @user_c @user_d
```

- is_show_count=True (with count display)

```txt
Information
  📝: <Link to message>

Users (4):
  @user_a @user_b @user_c @user_d
```

- is_author_include=True (including message author)

```txt
Information
  📝: <Link to message>
  🧔: @user_a 

Users:
  @user_a @user_b @user_c @user_d
```

- is_reaction_grouping=True and is_show_count=True

```txt
Information
  📝: <Link to message>

Reactions:
  👍 (2): @user_a @user_b
  ❤️ (1): @user_c
  😂 (2): @user_c @user_d
```

#### Error Cases

- When there are no reactions to the message

```
Information
  📝: <Link to message>
  🧔: <@user_a> 

Reactions:
  No one reacted.
```

- When the message does not exist, is not accessible, or has been deleted

```
  📝: <Parameter message>

⚠️ The message cannot be read.
- The message does not exist.
- You do not have permission to read the message.
- The message has been deleted.
```

- When there are many reactions

```aiignore
Due to the large number of reactions, it takes time to compile the results.
```

※This message is displayed when it cannot be returned within 3 seconds of interaction, and a normal message is sent separately after aggregation.

#### When called from message context menu

There are two context menu options:
1. **"Get reaction members"**: Default settings applied (is_reaction_grouping=false, is_author_include=false, is_show_count=false)
2. **"Get reaction-grouping members"**: Reaction grouping enabled (is_reaction_grouping=true, is_author_include=false, is_show_count=false)

## Bot Installation Target

- ◯: User
- ◯: Server (guild)

## Setup

### Prerequisites

- Rust 1.88.0 or higher
- Application creation in Discord Developer Portal

### Installation Steps

1. Clone repository
2. Configure `.env` file
3. Run with `cargo run`

### Deployment

#### Target Execution Environment

**Local Environment**
- For development and testing
- Small-scale operation on personal servers
- Memory: minimum 256MB, recommended 512MB
- CPU: 1 core or more

**VPS/Cloud Environment**
- Medium-scale server operation
- When 24-hour operation is required
- Memory: minimum 512MB, recommended 1GB
- CPU: 1-2 cores
- Storage: 1GB or more free space

**Docker Environment**

```dockerfile
FROM rust:1.88-slim as builder
WORKDIR /app
COPY .. .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/discord_reaction_info_selenity /usr/local/bin/
CMD ["discord_reaction_info_selenity"]
```

**Docker Compose**
```yaml
version: '3.8'
services:
  discord-bot:
    build: .
    environment:
      - DISCORD_TOKEN=${DISCORD_TOKEN}
    restart: unless-stopped
```

#### Required Resources

**Minimum Requirements**
- RAM: 256MB
- CPU: 1 core
- Disk: 500MB
- Network: Stable internet connection

**Recommended Requirements**
- RAM: 512MB-1GB
- CPU: 1-2 cores
- Disk: 1GB
- Network: Low-latency internet connection

#### Deployment Steps

1. **Environment variable setup for production**
   ```bash
   export DISCORD_TOKEN="your_token_here"
   export RUST_LOG="info"
   ```

2. **systemd service setup (Linux)**
   ```ini
   [Unit]
   Description=Discord Reaction Info Bot
   After=network.target

   [Service]
   Type=simple
   User=discord-bot
   WorkingDirectory=/opt/discord-bot
   ExecStart=/opt/discord-bot/discord_reaction_info_selenity
   Environment=DISCORD_TOKEN=your_token_here
   Environment=RUST_LOG=info
   Restart=always
   RestartSec=10

   [Install]
   WantedBy=multi-user.target
   ```

3. **Service startup**
   ```bash
   sudo systemctl enable discord-bot
   sudo systemctl start discord-bot
   ```

## Troubleshooting

### Common Problems and Solutions

#### Bot won't start

**Symptoms**: Bot doesn't start when running `cargo run`

**Causes and Solutions**:
1. **Invalid token**
   - Check `DISCORD_TOKEN` in `.env` file
   - Regenerate token in Discord Developer Portal

2. **Insufficient permissions**
   - Check if Bot has required permissions
   - Re-invite Bot with server admin permissions

3. **Network connection error**
   - Check internet connection
   - Check firewall settings

#### Commands don't respond

**Symptoms**: No response when executing `/reaction_members` command

**Causes and Solutions**:
1. **Slash commands not registered**
   - Command registration may take several minutes after Bot restart
   - Check command list by typing `/` in Discord

2. **Insufficient permissions**
   - Check message read permissions
   - Check channel access permissions

3. **Message specification error**
   - Check if message URL or ID is correct
   - Check if message has been deleted

#### Performance Issues

**Symptoms**: Command execution is slow or times out

**Causes and Solutions**:
1. **Large number of reactions**
   - Processing time increases with more than 100 reactions
   - Use filtering to reduce scope

2. **Large number of users**
   - When more than 1000 users have reacted
   - Use filtering parameters to narrow down

3. **Discord API rate limiting**
   - When executing many commands in a short time
   - Wait a few minutes before retrying

#### Memory shortage errors

**Symptoms**: Bot crashes, memory-related errors

**Causes and Solutions**:
1. **Resource shortage**
   - Increase server memory
   - Stop other processes to free resources

2. **Memory leaks**
   - Restart Bot regularly
   - Check logs for abnormal behavior

### Log Checking Methods

**Log level settings**
```bash
export RUST_LOG=debug  # Detailed logs
export RUST_LOG=info   # Normal logs
export RUST_LOG=error  # Errors only
```

**Log output checking**
```bash
# Check in standard output
cargo run

# Output to file
cargo run > bot.log 2>&1

# Check systemd logs
journalctl -u discord-bot -f
```

### Support Information

When problems cannot be resolved:

1. **GitHub Issues**: Bug reports and feature requests
2. **Discord Support Server**: Real-time support (in preparation)
3. **Documentation**: Detailed information in `docs/` folder

**Information to include when reporting problems**:
- Environment (OS, Rust version)
- Error messages (logs)
- Commands and parameters executed
- Expected behavior vs actual behavior