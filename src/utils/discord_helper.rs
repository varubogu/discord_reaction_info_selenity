use std::collections::HashMap;
use serenity::all::{CommandInteraction, CreateInteractionResponseFollowup, Message, User, UserId};
use serenity::client::Context;
use serenity::http::Http;

pub async fn fetch_event_message(ctx: &Context, command: &CommandInteraction) -> Result<Message, String> {
    let message = command.data.resolved.messages.values().next().cloned();

    match message {
        Some(msg) => Ok(msg),
        None => { // 処理中にメッセージが無くなった場合
            Err("⚠️ Could not find the target message.".to_string())
        }
    }
}

pub async fn fetch_discord_users(
    http: &Http,
    user_ids: &[UserId],
) -> HashMap<UserId, Result<User, serenity::Error>> {
    let mut results: HashMap<UserId, Result<User, serenity::Error>> = HashMap::new();
    
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
pub async fn make_message_url(message: &Message) -> String {
    
    format!("https://discord.com/channels/{}/{}/{}",
        // サーバーIDが無い場合、個人DMまたはグループDMとして「@me」を使用
        message.guild_id.map(|id| id.to_string()).unwrap_or_else(|| "@me".to_string()),
        message.channel_id,
        message.id
    )
}