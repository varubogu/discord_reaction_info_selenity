use anyhow::Result;
use regex::Regex;
use crate::utils::url_parser::{is_url, try_parse_discord_url, IdType};

/// Parse user mentions from a string containing mentions or user IDs
#[allow(dead_code)]
pub async fn parse_user_mentions(input: &str) -> Vec<u64> {
    let re = Regex::new(r"<@!?(\d+)>").unwrap();

    // 全てのマッチを取得し、キャプチャグループ1（数字部分）を収集
    re.captures_iter(input)
        .filter_map(|cap| cap.get(1))
        .filter_map(|m| m.as_str().parse::<u64>().ok())
        .collect()
}

/// Parse reactions from a string containing reaction emojis or names
#[allow(dead_code)]
pub async fn parse_reactions(input: &str) -> Vec<String> {
    input.split(&[' ', ','])
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Parse message identifier (URL or ID) and return (guild_id, channel_id, message_id)
#[allow(dead_code)]
pub async fn parse_message_identifier(input: &str) -> Result<u64> {
    // Check if it's a Discord message URL
    if is_url(input).await {
        let result = try_parse_discord_url(input).await;
        return Ok(*result.unwrap().get(&IdType::MessageId).unwrap())
    }

    // Check if it's just a message ID (all digits)
    if input.chars().all(|c| c.is_ascii_digit()) {
        let message_id: u64 = input.parse()?;
        return Ok(message_id);
    }
    
    Err(anyhow::anyhow!("Invalid message identifier"))
}