# Development

## Development Environment

### Required Tools

**Basic Environment**
- Language: Rust 1.88.0 or higher
- Framework: Poise (Discord API client built on Serenity)
- Package Manager: Cargo

**Recommended Development Tools**
- Editor: VS Code, IntelliJ IDEA, vim/neovim
- VS Code Extensions: rust-analyzer, CodeLLDB
- Git: Version control
- Docker: Container environment testing

### Development Environment Setup

#### 1. Rust Installation

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**
1. Download rustup-init.exe from https://rustup.rs/
2. Run the installer
3. Restart PowerShell or Command Prompt

**Version Check:**
```bash
rustc --version
cargo --version
```

#### 2. Project Clone and Configuration

```bash
# Clone repository
git clone https://github.com/varubogu/discord_reaction_info_selenity.git
cd discord_reaction_info_selenity

# Install dependencies
cargo build

# Run tests to verify setup
cargo test
```

#### 3. Discord Bot Configuration

**Discord Developer Portal Setup:**
1. Go to https://discord.com/developers/applications
2. Click "New Application"
3. Enter application name
4. Go to "Bot" tab
5. Click "Add Bot"
6. Copy token (use later)

**Environment Variable Setup:**
```bash
# Create .env file
cp .env.example .env

# Edit .env file
echo "DISCORD_TOKEN=your_bot_token_here" > .env
echo "RUST_LOG=debug" >> .env  # Enable detailed logs for development
```

#### 4. Bot Invitation to Development Server

**OAuth2 URL Generation:**
1. Discord Developer Portal OAuth2 tab
2. Select "URL Generator"
3. Scopes: `bot`, `applications.commands`
4. Bot Permissions:
   - Read Message History
   - Send Messages
   - Use Slash Commands
   - Read Message Reactions
5. Invite to test server with generated URL

### Development Workflow

#### Local Development

```bash
# Run in development mode (no hot reload)
cargo run

# Build in release mode
cargo build --release

# Run specific tests
cargo test test_name

# Generate documentation
cargo doc --open
```

#### Code Formatting and Linting

```bash
# Code formatting
cargo fmt

# Run linter
cargo clippy

# Strict linting
cargo clippy -- -D warnings
```

#### Debugging Methods

**Log Output Level Setting:**
```bash
# Detailed debug logs
export RUST_LOG=debug

# Specific module only
export RUST_LOG=discord_reaction_info_selenity=debug

# Multiple level specification
export RUST_LOG=debug,poise=info
```

**Debugger Usage:**
- VS Code: Use CodeLLDB extension
- Command line: `rust-gdb target/debug/discord_reaction_info_selenity`

#### Performance Analysis

```bash
# Build with profiling
cargo build --profile release-with-debug

# Run benchmarks
cargo bench

# Memory usage analysis
valgrind --tool=massif target/release/discord_reaction_info_selenity
```

## Technical Specifications

### Libraries Used

- tokio: Asynchronous runtime
- poise: Discord bot framework (built on Serenity)
- anyhow: Error handling
- tracing: Log output
- dotenv: Environment variable management

### Required Discord Bot Permissions

- Read Message History
- Send Messages
- Read Message Reactions
- Use Slash Commands

### Environment Variables

- `DISCORD_TOKEN`: Bot's Discord token

## Limitations

- Comply with Discord API rate limits
- Processing time for messages with many reactions
- Message retention period constraints
- Concurrent execution limits

## Performance Considerations

- Maximum reactions per message: 100
- Maximum users per reaction: 1000
- Maximum users per message: 10000
- Command execution timeout: 15 seconds
- Concurrent execution limit: 1 per user

## Notes

- Operating restrictions in private channels
- Handling of deleted users
- Performance considerations on large servers

## Coding Standards

- Maximum 80 characters per line
- Maximum 50 lines per function
- Add docstrings to all documentable items (structs, traits, functions, etc.)
- Maximum 200 lines per file (excluding test modules)
- Nesting: Maximum 3 levels within functions
- Follow DRY and SOLID principles
- Unit tests in same file test modules
- Integration tests in `tests/integration/**.rs`
- System tests in `tests/system/**.rs`
- Follow standard Rust coding conventions

## Project Structure

```
(root)
├── .env          # Environment variables (local)
├── Cargo.toml    # Cargo crate definition
├── Cargo.lock    # Cargo installed crate definition
├── docs/         # Documentation
├── src/          # Source code
│   ├── main.rs   # Entry point to event loop logic
│   ├── init.rs   # Bot startup initialization processing
│   ├── events/                 # All events ※Define only entry points, actual logic placed in src/services/
│   │   ├── mod.rs              # Module declaration
│   │   ├── on_message.rs       # Message sent listener
│   │   ├── on_reaction_add.rs  # Message reaction listener
│   │   ├── xxxxx.rs            # Event listeners, file name same as event name
│   │   ├── interactions/       # All interactions
│   │   │   ├── command_interactions/     # Command interactions
│   │   │   │   ├── mod.rs                # Module declaration
│   │   │   │   ├── slash/                # Slash command definitions
│   │   │   │   │   ├── mod.rs            # Module declaration
│   │   │   │   │   ├── reaction_members.rs     # Slash command to collect and display reaction user info
│   │   │   │   │   ├── xxxx_slash.rs     # Slash commands end with "_slash.rs"
│   │   │   │   ├── contextmenu/                          # Context menu interactions
│   │   │   │   │   ├── mod.rs                            # Module declaration
│   │   │   │   │   ├── reaction_users_context_menu.rs    # Context menu to collect and display reaction user info
│   │   │   │   │   ├── reaction_grouping_users_context_menu.rs # Context menu for grouped reaction user info
│   │   │   │   │   ├── xxxx_context_menu.rs              # Context menus end with "_context_menu.rs"
│   │   │   ├── components/             # Component interactions
│   │   │   │   ├── xxxx_component.rs   # Components end with "_component.rs"
│   │   │   ├── modal/                  # Modal submission interactions
│   │   │   │   ├── xxxx_modal.rs       # Modals end with "_modal.rs"
│   ├── services/               # Business logic, consider folder separation for larger scale
│   │   ├── mod.rs              # Module declaration
│   │   ├── reaction_users.rs   # Collect reaction user info and return results
│   │   ├── xxx.rs              # Business logic
│   ├── utils/        # General purpose processing folder
│   │   ├── mod.rs    # Module declaration
│   │   ├── xxx.rs    # General purpose processing
├── tests/            # Test code
│   ├── integration/  # Integration tests
│   ├── system/       # System tests
```