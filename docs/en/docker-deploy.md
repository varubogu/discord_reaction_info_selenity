# Docker Deployment

This project can be deployed using Docker and Docker Compose.

## Requirements

- Docker
- Docker Compose
- Discord Bot Token

## Usage

### 1. Environment Variable Setup

```bash
# Create .env file (copy from existing .env.example)
cp .env.example .env

# Edit .env file to set DISCORD_TOKEN
echo "DISCORD_TOKEN=your_discord_bot_token_here" > .env
```

### 2. Starting with Docker Compose

```bash
# Start in background
docker-compose up -d

# View logs
docker-compose logs -f

# Stop
docker-compose down
```

### 3. Starting with Individual Docker Commands

```bash
# Build image
docker build -t discord-reaction-bot .

# Start container
docker run -d --name discord-bot --env-file .env --restart unless-stopped discord-reaction-bot

# View logs
docker logs -f discord-bot

# Stop
docker stop discord-bot
docker rm discord-bot
```

## Resource Requirements

- **Minimum Requirements**: RAM 256MB, CPU 1 core
- **Recommended Requirements**: RAM 512MB-1GB, CPU 1-2 cores

## Troubleshooting

### When Container Won't Start

1. Verify DISCORD_TOKEN is set correctly
2. Check if Docker is running
3. Check logs: `docker-compose logs discord-bot`

### When Bot Doesn't Respond

1. Check bot permissions in Discord Developer Portal
2. Verify bot is invited to the server
3. Check network connection