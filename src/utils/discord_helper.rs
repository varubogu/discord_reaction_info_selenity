use std::collections::HashMap;
use poise::serenity_prelude::{ChannelId, Http, Message, MessageId, User, UserId};
use crate::Context;
use crate::utils::url_parser::{is_url, try_parse_discord_url, IdType};

#[allow(dead_code)]
pub async fn fetch_discord_users(
    http: &Http,
    user_ids: &[UserId],
) -> HashMap<UserId, Result<User, poise::serenity_prelude::Error>> {
    let mut results: HashMap<UserId, Result<User, poise::serenity_prelude::Error>> = HashMap::new();
    
    for user_id in user_ids {
        results.insert(*user_id, http.get_user(*user_id).await);
    }
    results
}


/// Constructs a Discord message URL based on the provided `Message` object.
///
/// # Parameters
/// - `message`: A reference to a `Message` object that contains the necessary data to
///   construct the URL, including `guild_id`, `channel_id`, and `message_id`.
///
/// # Returns
/// - A `String` representing the URL of the specific Discord message. The format of the URL is:
///   `https://discord.com/channels/{guild_id}/{channel_id}/{message_id}`
///
///   - If the `guild_id` is present, it is included in the URL.
///   - If the `guild_id` is absent (e.g., for direct messages), the URL substitutes `@me` in place of the `guild_id`.
///
/// # Example
/// ```
/// let message = Message {
///     guild_id: Some(123456789012345678),
///     channel_id: 234567890123456789,
///     id: 345678901234567890,
/// };
///
/// let url = make_message_url(&message);
/// assert_eq!(url, "https://discord.com/channels/123456789012345678/234567890123456789/345678901234567890");
///
/// let dm_message = Message {
///     guild_id: None,
///     channel_id: 234567890123456789,
///     id: 345678901234567890,
/// };
///
/// let dm_url = make_message_url(&dm_message);
/// assert_eq!(dm_url, "https://discord.com/channels/@me/234567890123456789/345678901234567890");
/// ```
///
/// # Notes
/// - This function assumes that the `Message` struct has fields `guild_id`, `channel_id`, and `id`,
///   where `guild_id` is an `Option` type.
#[allow(dead_code)]
pub async fn make_message_url(message: &Message) -> String {
    
    format!("https://discord.com/channels/{}/{}/{}",
        // サーバーIDが無い場合、個人DMまたはグループDMとして「@me」を使用
        message.guild_id.map(|id| id.to_string()).unwrap_or_else(|| "@me".to_string()),
        message.channel_id,
        message.id
    )
}

#[allow(dead_code)]
pub async fn parse_message_context(ctx: Context<'_>, message_id_or_url: String) -> Result<Message, Vec<String>> {
    if is_url(&message_id_or_url).await {
        let parsed_data = try_parse_discord_url(&message_id_or_url)
            .await
            .map_err(|e| vec![e.to_string()])?;

        let channel_id_u64 = parsed_data.get(&IdType::ChannelId);
        let message_id_u64 = parsed_data.get(&IdType::MessageId);

        // チャンネルID、メッセージIDが取得できなければエラー
        if channel_id_u64.is_none() || message_id_u64.is_none() {
            return Err(vec!["Invalid URL format or missing channel/message ID.".to_string()]);
        }

        let channel_id = ChannelId::from(*channel_id_u64.unwrap());
        let message_id = MessageId::from(*message_id_u64.unwrap());

        // チャンネルIDがコンテキストと一致するか確認
        let context_channel_id = ctx.channel_id();
        if channel_id != context_channel_id {
            return Err(vec!["Invalid URL format or missing channel/message ID.".to_string()]);
        }

        get_message(ctx, channel_id, message_id).await
    } else if let Ok(message_id) = message_id_or_url.parse::<u64>() {
        let channel_id = ctx.channel_id();
        let message_id = MessageId::from(message_id);
        get_message(ctx, channel_id, message_id).await
    } else {
        Err("Invalid message ID or URL format.".to_string())
    }.expect("TODO: panic message");

    Err(vec!["Invalid message ID or URL format.".to_string()])
}

#[allow(dead_code)]
async fn get_message(ctx: Context<'_>, channel_id: ChannelId, message_id: MessageId) -> Result<Message, String> {
    let http = ctx.http();
    match http.get_message(channel_id, message_id).await {
        Ok(message) => Ok(message),
        Err(e) => return Err(format!("Failed to fetch message: {}", e))
    }
}