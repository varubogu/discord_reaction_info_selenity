# Discord Reaction Info Bot (Poise)

A Discord Bot that extracts and displays reaction information from Discord messages. Built with Rust and the Poise framework (built on Serenity), this bot provides convenient slash commands and context menus to analyze message reactions.

English | [日本語](README.ja.md)

## Features

- **Reaction Analysis**: Extract detailed reaction information from any Discord message
- **Multiple Access Methods**: Available via slash commands and context menus
- **Flexible Filtering**: Include/exclude specific users, roles, or reaction types
- **Multiple Display Modes**: Show reactions by type, user lists, or summary counts
- **Multilingual Support**: Supports Japanese and English
- **Privacy Focused**: Results are shown only to the command user

## Commands

### `/reaction_members` - Reaction Members Command

Analyze reactions on a specific message and display the information.

**Usage:**
```
/reaction_members message:<message_url_or_id> [is_author_include:true/false] [is_show_count:true/false] [is_reaction_grouping:true/false]
```

**Parameters:**
- `message` (required): Message URL or Message ID
- `is_author_include` (optional): Include message author in results (default: false)
- `is_show_count` (optional): Show reaction counts in results (default: false)
- `is_reaction_grouping` (optional): Group users by reaction type (default: false)
  - `true`: Show users grouped by each reaction type
  - `false`: Show all reaction users combined (deduplicated)

### Context Menus

Right-click any message and choose from two options:
- **"Get reaction members"**: Quick reaction analysis with default settings (combined reactions)
- **"Get reaction-grouping members"**: Quick reaction analysis grouped by reaction type

## Installation

### Prerequisites

- Rust 1.88.0 or higher
- Discord Application created in Discord Developer Portal
- Discord Bot Token

### Setup Steps

1. **Clone the repository:**
   ```bash
   git clone https://github.com/varubogu/discord_reaction_info_selenity.git
   cd discord_reaction_info_selenity
   ```

2. **Create environment file:**
   ```bash
   cp .env.example .env
   ```
   
3. **Configure your Discord token:**
   Edit `.env` and add your Discord bot token:
   ```
   DISCORD_TOKEN=your_bot_token_here
   ```

4. **Build and run:**
   ```bash
   cargo build --release
   cargo run
   ```

## Discord Bot Setup

### Required Permissions

The bot needs the following Discord permissions:
- Read Message History
- Send Messages
- Use Slash Commands
- Read Message Reactions

### Adding to Your Server

1. Go to Discord Developer Portal
2. Select your application
3. Go to OAuth2 > URL Generator
4. Select "bot" and "applications.commands" scopes
5. Select the required permissions above
6. Use the generated URL to invite the bot to your server

## Usage Examples

### Basic Usage
```
/reaction_members message:1234567890123456789
```

### Show Reaction Counts
```
/reaction_members message:https://discord.com/channels/111/222/333 is_show_count:true
```

### Group by Reaction Type
```
/reaction_members message:1234567890123456789 is_reaction_grouping:true
```

### Include Message Author
```
/reaction_members message:1234567890123456789 is_author_include:true
```

## Performance Limits

- Maximum 100 reactions per message
- Maximum 1000 users per reaction
- 15-second command timeout
- Rate limiting compliance with Discord API

## Development

See [docs/develop.md](docs/en/develop.md) for detailed development information, coding standards, and project structure.

## Documentation

- [Project Specifications](docs/en/spec.md) - Detailed command specifications and examples
- [Development Guide](docs/en/develop.md) - Development environment and coding standards
- [FAQ](docs/en/faq.md) - Frequently asked questions and troubleshooting

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

If you encounter any issues or have questions:
1. Check the documentation in the `docs/` folder
2. Review the examples above
3. Open an issue on GitHub

## Contributing

Contributions are welcome! Please refer to the development guide in `docs/develop.md` for coding standards and project structure guidelines.
