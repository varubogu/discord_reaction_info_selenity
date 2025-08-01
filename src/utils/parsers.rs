use anyhow::Result;

/// Parse user mentions from a string containing mentions or user IDs
pub fn parse_user_mentions(input: &str) -> Vec<u64> {
    let mut user_ids = Vec::new();
    
    // Split by spaces and commas
    for part in input.split(&[' ', ',']) {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        
        // Parse mention format <@123456789> or <@!123456789>
        if part.starts_with("<@") && part.ends_with(">") {
            let id_part = &part[2..part.len()-1];
            let id_part = if id_part.starts_with("!") { &id_part[1..] } else { id_part };
            if let Ok(id) = id_part.parse::<u64>() {
                user_ids.push(id);
            }
        }
        // Parse raw user ID
        else if let Ok(id) = part.parse::<u64>() {
            user_ids.push(id);
        }
    }
    
    user_ids
}

/// Parse reactions from a string containing reaction emojis or names
pub fn parse_reactions(input: &str) -> Vec<String> {
    input.split(&[' ', ','])
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Parse message identifier (URL or ID) and return (guild_id, channel_id, message_id)
pub fn parse_message_identifier(input: &str) -> Result<u64> {
    // Check if it's a Discord message URL
    if input.starts_with("https://discord.com/channels/") || input.starts_with("https://discordapp.com/channels/") {
        let parts: Vec<&str> = input.split('/').collect();
        if parts.len() >= 7 {
            let message_id = parts[7].parse()?;
            return Ok(message_id);
        }
    }
    
    // Check if it's just a message ID (all digits)
    if input.chars().all(|c| c.is_ascii_digit()) {
        let message_id: u64 = input.parse()?;
        return Ok(message_id);
    }
    
    Err(anyhow::anyhow!("Invalid message identifier"))
}