# Frequently Asked Questions (FAQ)

## General Questions

### Q: What does this Bot do?
A: This is a Discord Bot that analyzes and displays reaction information from Discord messages. You can easily check which users gave which reactions to specific messages.

### Q: Can I use it for free?
A: Yes, this Bot is completely free and open source. You can use it freely for personal and commercial purposes.

### Q: In what situations would I use this?
A: It's useful in situations such as:
- Checking event participants (participation/non-participation emoji reactions)
- Collecting survey results (multiple choice reactions)
- Checking voting results
- Community reaction analysis

## Usage Instructions

### Q: Where can I get the message URL?
A: 
1. Right-click the target message
2. Select "Copy Message Link"
3. Use the copied URL with the `/reaction_members` command

### Q: Can I use just the message ID?
A: Yes, you can also use the message ID. You can get the ID by enabling developer mode and right-clicking the message → "Copy ID".

### Q: Can I analyze messages from other servers?
A: No, you can only analyze messages from servers where the Bot is participating. You also need to have access permissions to that message yourself.

### Q: Can I analyze private messages (DMs)?
A: No, private message analysis is not supported. Only public channel messages within servers are supported.

## Troubleshooting

### Q: Nothing happens when I enter a command
A: Please check the following:
1. Is the Bot properly added to the server?
2. Does the Bot have the required permissions?
3. Does `/reaction_members` appear in the command list when you type `/`?
4. If the Bot was recently restarted, wait a few minutes and try again

### Q: "Cannot read message" is displayed
A: The following causes are possible:
1. The message has been deleted
2. You or the Bot don't have access permissions to the message
3. The message URL or ID is incorrect
4. The target message is too old (exceeds Discord's retention period)

### Q: "Processing takes time due to large number of reactions" is displayed
A: When there are many reactions (100+) or many users (1000+), processing time increases. You can reduce processing time by:
- Using filtering parameters to narrow down the scope
- Breaking down into smaller ranges for execution

### Q: Timeout error occurs
A: Processing that exceeds 15 seconds will timeout. Try the following:
1. Use filtering parameters to narrow down targets
2. Wait some time before retrying (possible Discord API rate limiting)
3. Execute in smaller, divided ranges

## Permissions

### Q: What are the minimum permissions required for the Bot?
A: 
- Read Message History
- Send Messages
- Use Slash Commands
- Read Message Reactions

### Q: Are administrator permissions required?
A: No, it works with only the basic permissions above. For security reasons, we recommend not granting more permissions than necessary.

### Q: Can I restrict usage to specific channels only?
A: Yes, you can enable Bot permissions only for specific channels through Discord server permission settings.

## Performance and Limitations

### Q: Can I execute multiple commands simultaneously?
A: There's a limit of 1 concurrent execution per user. Please wait for the previous command to complete before executing the next one.

### Q: Can I use it on large servers?
A: Yes, but there are the following limitations:
- Maximum 100 reactions per message
- Maximum 1000 users per reaction
- Command execution timeout: 15 seconds

### Q: Are there rate limits?
A: It follows Discord API rate limits. If you execute many commands in a short time, you may be temporarily limited.

## Setup and Installation

### Q: How do I add the Bot to my server?
A: 
1. Clone the project and set up the environment
2. Create a Bot application in Discord Developer Portal
3. Get and configure the token
4. See [docs/develop.md](develop.md) for details

### Q: Can I deploy to VPS or cloud?
A: Yes, you can deploy to Linux VPS, AWS, Google Cloud, etc. See [docs/spec.md](spec.md) for detailed deployment instructions.

### Q: Can I run it with Docker?
A: Yes, it supports Docker and Docker Compose. Configuration examples are provided in [docs/spec.md](spec.md).

## Development and Customization

### Q: Can I customize the code?
A: Yes, it's an open source project so you can freely customize it. See [docs/develop.md](develop.md) for development guidelines.

### Q: Can I request new features?
A: You can submit feature requests via GitHub Issues. Pull request contributions are also welcome.

### Q: What should I do if I find a bug?
A: Please submit a bug report via GitHub Issues. Including reproduction steps and error messages enables quick response.

## Other

### Q: Does it support other languages?
A: Currently supports Japanese and English. Other language support can be discussed on GitHub.

### Q: Can I use it commercially?
A: Yes, commercial use is possible under the MIT License. See the [LICENSE](../../LICENSE) file for details.

### Q: Where can I get support?
A: 
1. Documentation in docs/ folder
2. GitHub Issues (bug reports and feature requests)
3. GitHub Discussions (general questions)

---

**If you have problems not resolved by this FAQ, please ask via GitHub Issues.**