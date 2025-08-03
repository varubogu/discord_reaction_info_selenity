use crate::services::reaction_users::types::{ReactionUsersParameter, ReactionUsersResponse};
use crate::services::reaction_users::utils::to_reaction_map;
use crate::utils::discord_helper::make_message_url;
use anyhow::Result;
use serenity::all::User;
use serenity::{
    model::channel::Message,
    prelude::*,
};
use std::collections::HashMap;

pub mod types;
pub mod utils;

pub async fn process_reaction_members(
    ctx: &Context,
    message: &Message,
    parameter: &ReactionUsersParameter,
) -> Result<ReactionUsersResponse> {

    let header_text = get_reaction_users_header_text(message).await?;

    if message.reactions.is_empty() {
        // メッセージにリアクションがない場合
        let text = get_reaction_empty_text().await?;
        Ok(ReactionUsersResponse {
            content: header_text + &text
        })
    } else if parameter.is_unique_users {
        // 全てのリアクションを合算してユーザーを取得
        let text = get_reaction_unique_users_text(ctx, message, parameter).await?;
        Ok(ReactionUsersResponse {
            content: header_text + &text
        })
    } else {
        // リアクションごとにユーザーを取得
        let text = get_reaction_users_text(ctx, message, parameter).await?;
        Ok(ReactionUsersResponse {
            content: header_text + &text
        })
    }
}

async fn get_reaction_users_header_text(
    message: &Message
) -> Result<String> {

    let mut result: String = "".to_string();

    let author_mention = message.author.mention();
    let message_url = make_message_url(message).await;

    // ヘッダ情報（メッセージリンク、発言者）
    result.push_str(&format!(r###"\
Information\
  📝: {}\
  🧔: {}\

Reactions:\
"###, message_url, author_mention));

    Ok(result.to_string())
}

async fn get_reaction_empty_text() -> Result<String> {
    Ok("No one reacted.".to_string())
}

/// Assembles a formatted string containing user IDs from a hashmap of mentions.
///
/// # Parameters
/// - `mentions`: A `HashMap` where the key is a `String`,
///   and the value is a `Vec` of `User` objects associated with the key.
///
/// # Returns
/// An `OK(String)` containing a Markdown-formatted code block with a space-separated list of user IDs
/// extracted from the `mentions` map. The IDs are aggregated from all values (vectors) in the hashmap.
/// If an error occurs during processing, it will be returned as a `Result::Err`.
///
/// # Examples
/// ```
/// use std::collections::HashMap;
///
/// #[derive(Debug)]
/// struct User {
///     id: u64,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut mentions: HashMap<String, Vec<User>> = HashMap::new();
///
///     mentions.insert(
///         String::from("reaction1"),
///         vec![User { id: 123 }, User { id: 456 }],
///     );
///
///     mentions.insert(
///         String::from("reaction2"),
///         vec![User { id: 789 }],
///     );
///
///     let result = reaction_mentions_user_id_only(mentions).await?;
///     assert_eq!(result, "```123 456 789```\n");
///     println!("{}", result);
///     Ok(())
/// }
/// ```
///
/// # Notes
/// - The `id` field of the `User` struct must be accessible through `.id`.
/// - The resulting string is wrapped in a Markdown-style code block enclosed with triple backticks (` ``` `).
/// - User IDs are joined with a single space as the delimiter.
///
/// # Errors
/// - This function will return `Result::Err` if an error occurs during string formatting or manipulation.
async fn get_reaction_unique_users_text(
    ctx: &Context,
    message: &Message,
    parameter: &ReactionUsersParameter
) -> Result<String> {

    // リアクションユーザーMap取得
    let mentions = to_reaction_map(&ctx, &message, &[]).await?;

    // リアクションごとのユーザーでフラット化
    let mut users: Vec<User> = mentions
        .into_values()
        .flat_map(|vec|vec.into_iter())
        .collect();

    // メッセージの発言者も含める
    if parameter.is_author_include {
        users.insert(0, message.author.clone());
    }

    let users = users
        .into_iter()
        .map(|x| format!("{}", x.mention()))
        .collect::<Vec<String>>()
        .join(" ");

    Ok(format!("```{}```\n", users))
}

async fn get_reaction_users_text(
    ctx: &Context,
    message: &Message,
    parameter: &ReactionUsersParameter
) -> Result<String> {

    // リアクションユーザーMap取得
    let mentions = to_reaction_map(&ctx, &message, &[]).await?;

    let results = mentions
        .into_iter()
        .map(|(emoji, users)| {
            let count = users.len();
            let user_mentions = users
                .into_iter()
                .map(|x| x.mention().to_string())
                .collect::<Vec<String>>()
                .join(" ");

            if parameter.is_show_count {
                format!("  {}: {:>4}: {}```{}```", emoji, count, user_mentions, user_mentions)
            } else {
                format!("  {}: {}```{}```", emoji, user_mentions, user_mentions)
            }
        })
        .collect::<Vec<String>>()
        .join(" ");

    Ok(results)
}

/// Generates a formatted string to list emoji reactions along with the users who reacted.
///
/// # Arguments
///
/// * `mentions` - A `HashMap` where the key is the emoji (as a `String`),
/// and the value is a `Vec<String>` containing the usernames of the users who reacted with that emoji.
/// * `is_show_count` - A `bool` flag indicating whether to include the count of reactions for each emoji.
///
/// # Returns
///
/// Returns a `Result<String>` where the `String` contains the formatted output.
/// Each line of the output contains:
/// - The emoji.
/// - (Optional) The count of reactions, if `is_show_count` is true.
/// - A formatted code block containing the list of usernames who reacted with that emoji.
///
/// # Errors
///
/// Returns an error if there are unforeseen issues during processing (though no explicit errors are handled in this function).
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut mentions = HashMap::new();
///     mentions.insert("👍".to_string(), vec!["user1".to_string(), "user2".to_string()]);
///     mentions.insert("❤️".to_string(), vec!["user3".to_string()]);
///
///     let result = reaction_mentions(mentions, true).await?;
///
///     println!("{}", result);
///
///     Ok(())
/// }
/// ```
///
/// Example Output (if `is_show_count` is `true`):
/// ```
///   👍:    2: user1 user2```user1 user2```
///   ❤️:    1: user3```user3```
/// ```
///
/// Example Output (if `is_show_count` is `false`):
/// ```
///   👍: user1 user2```user1 user2```
///   ❤️: user3```user3```
/// ```
async fn reaction_mentions(
    mentions: HashMap<String, Vec<User>>,
    parameter: &ReactionUsersParameter
) -> Result<String> {
    let mut result = String::new();

    let format_func = if parameter.is_show_count {
        |emoji: &str, count: usize, users: String| format!("  {}: {:>4}: {}```{}```\n", emoji, count, users, users)
    } else {
        |emoji: &str, _count: usize, users: String| format!("  {}: {}```{}```\n", emoji, users, users)
    };

    for (emoji, users) in mentions {
        let count = users.len();
        let u = users
            .into_iter()
            .map(|x| format!("{}", x.id))
            .collect::<Vec<String>>()
            .join(" ");

        result.push_str(&format_func(&emoji, count, u.to_string().to_string()));
    }
    Ok(result)
}
