use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::HashMap;


/// URLかどうかを判定する
pub async fn is_url(url: &str) -> bool {
    url.starts_with("https://")
}

/// Discord URLを解析して構造化データを返す
pub async fn try_parse_discord_url(url: &str) -> Result<HashMap<String, u64>, String> {
    let pattern = r"https://discord\.com/channels/(\d+)/(\d+)/(\d+)";
    let re = Regex::new(pattern).map_err(|_| "正規表現のコンパイルに失敗しました。".to_string())?;

    if let Some(captures) = re.captures(url) {
        let mut result = HashMap::new();

        let guild_id = captures.get(1)
            .ok_or("guild_idの取得に失敗しました。")?
            .as_str()
            .parse::<u64>()
            .map_err(|_| "guild_idの数値変換に失敗しました。")?;

        let channel_id = captures.get(2)
            .ok_or("channel_idの取得に失敗しました。")?
            .as_str()
            .parse::<u64>()
            .map_err(|_| "channel_idの数値変換に失敗しました。")?;

        let message_id = captures.get(3)
            .ok_or("message_idの取得に失敗しました。")?
            .as_str()
            .parse::<u64>()
            .map_err(|_| "message_idの数値変換に失敗しました。")?;

        result.insert("guild_id".to_string(), guild_id);
        result.insert("channel_id".to_string(), channel_id);
        result.insert("message_id".to_string(), message_id);

        Ok(result)
    } else {
        Err("URLの解析に失敗しました。".to_string())
    }
}

/// URLからguild_idを取得する
pub async fn try_parse_guild_id(url: &str) -> Result<u64, String> {
    let pattern = r"https://discord\.com/channels/(\d+)/\d+/\d+";
    let re = Regex::new(pattern).map_err(|_| "正規表現のコンパイルに失敗しました。".to_string())?;

    if let Some(captures) = re.captures(url) {
        captures.get(1)
            .ok_or("guild_idの取得に失敗しました。")?
            .as_str()
            .parse::<u64>()
            .map_err(|_| "URLからguild_idの取得に失敗しました。".to_string())
    } else {
        Err("URLからguild_idの取得に失敗しました。".to_string())
    }
}

/// URLからchannel_idを取得する
pub async fn try_parse_channel_id(url: &str) -> Result<u64, String> {
    let pattern = r"https://discord\.com/channels/\d+/(\d+)/\d+";
    let re = Regex::new(pattern).map_err(|_| "正規表現のコンパイルに失敗しました。".to_string())?;

    if let Some(captures) = re.captures(url) {
        captures.get(1)
            .ok_or("channel_idの取得に失敗しました。")?
            .as_str()
            .parse::<u64>()
            .map_err(|_| "URLからchannel_idの取得に失敗しました。".to_string())
    } else {
        Err("URLからchannel_idの取得に失敗しました。".to_string())
    }
}

/// URLからmessage_idを取得する
pub async fn try_parse_message_id(url: &str) -> Result<u64, String> {
    let pattern = r"https://discord\.com/channels/\d+/\d+/(\d+)";
    let re = Regex::new(pattern).map_err(|_| "正規表現のコンパイルに失敗しました。".to_string())?;

    if let Some(captures) = re.captures(url) {
        captures.get(1)
            .ok_or("message_idの取得に失敗しました。")?
            .as_str()
            .parse::<u64>()
            .map_err(|_| "URLからmessage_idの取得に失敗しました。".to_string())
    } else {
        Err("URLからmessage_idの取得に失敗しました。".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_is_url() {
        assert!(is_url("https://discord.com/channels/123/456/789").await);
        assert!(!is_url("http://example.com").await);
        assert!(!is_url("not a url").await);
    }

    #[tokio::test]
    async fn test_try_parse_discord_url() {
        let url = "https://discord.com/channels/123456789/987654321/111222333";
        let result = try_parse_discord_url(url).await.unwrap();
        
        assert_eq!(result["guild_id"], 123456789);
        assert_eq!(result["channel_id"], 987654321);
        assert_eq!(result["message_id"], 111222333);
    }

    #[tokio::test]
    async fn test_try_parse_guild_id() {
        let url = "https://discord.com/channels/123456789/987654321/111222333";
        let result = try_parse_guild_id(url).await.unwrap();
        
        assert_eq!(result, 123456789);
    }

    #[tokio::test]
    async fn test_try_parse_channel_id() {
        let url = "https://discord.com/channels/123456789/987654321/111222333";
        let result = try_parse_channel_id(url).await.unwrap();
        
        assert_eq!(result, 987654321);
    }

    #[tokio::test]
    async fn test_try_parse_message_id() {
        let url = "https://discord.com/channels/123456789/987654321/111222333";
        let result = try_parse_message_id(url).await.unwrap();
        
        assert_eq!(result, 111222333);
    }

    #[tokio::test]
    async fn test_invalid_url() {
        let invalid_url = "https://example.com/invalid";
        let result = try_parse_discord_url(invalid_url).await;
        
        assert!(result.is_err());
    }
}
