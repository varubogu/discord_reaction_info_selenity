# Project Guidelines

## Project Overview

This is a Discord Bot project written in Rust using the Selenity framework. The bot extracts Discord reaction information and provides it to users through slash commands and context menus. The main functionality is the `/rmem` (Reaction Members) command that collects and displays reaction information from Discord messages.

**Important: Always refer to the `docs/` folder for detailed specifications and development guidelines before making any changes.**

## Key Documentation

- `docs/spec.md` - Complete project specifications, command details, and usage examples
- `docs/develop.md` - Development environment, technical specifications, and coding conventions

## Project Structure

The project follows a modular structure:
- `src/main.rs` - Entry point and event loop
- `src/init.rs` - Bot initialization logic
- `src/events/` - Event handlers (message listeners, interactions)
- `src/services/` - Business logic implementation
- `src/lib/` - Utility functions and common code
- `tests/` - Test code (integration and system tests)

## Development Environment

- **Language**: Rust 1.88.0+
- **Framework**: Selenity (Discord API client)
- **Key Dependencies**: tokio, anyhow, tracing, dotenv
- **Environment Variables**: `DISCORD_TOKEN` required

## Coding Standards

**Always follow the coding conventions specified in `docs/develop.md`:**
- Maximum 80 characters per line
- Maximum 50 lines per function
- Maximum 200 lines per file (excluding test modules)
- Maximum 3 levels of nesting in functions
- Follow DRY and SOLID principles
- Adhere to standard Rust conventions

## Testing Requirements

- **Unit Tests**: Place in same file using `#[cfg(test)]` modules
- **Integration Tests**: Place in `tests/integration/**.rs`
- **System Tests**: Place in `tests/system/**.rs`
- **Always run tests** before submitting changes: `cargo test`

## Build and Deployment

- **Build**: Use `cargo build` to compile the project
- **Run**: Use `cargo run` to execute the bot
- **Always verify the build succeeds** before submitting changes

## Discord Bot Permissions Required

- Message history reading
- Message sending
- Reaction reading
- Slash command usage

## Performance Considerations

Refer to `docs/develop.md` for specific limits:
- Max 100 reactions per message
- Max 1000 users per reaction
- 15-second command timeout
- Rate limiting compliance

## File Naming Conventions

- Slash commands: `*_slash.rs`
- Context menus: `*_context_menu.rs`
- Components: `*_component.rs`
- Modals: `*_modal.rs`
