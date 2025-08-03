use std::collections::HashMap;
use serenity::all::{Message, User};
use serenity::client::Context;

/// Retrieves a mapping of reaction emojis to the users who reacted to them, while allowing certain reactions to be excluded.
///
/// # Parameters
/// - `ctx`: A reference to the [`Context`] which provides access to the Discord API and cache.
/// - `message`: A reference to the [`Message`] for which the reaction information is being retrieved.
/// - `exclude_reactions`: A slice of [`String`]s representing the emojis to exclude from the result.
///
/// # Returns
/// - On success, the `Ok` variant contains the mapping of reaction emojis to users.
/// - On failure, the `Err` variant may contain an error specifying what went wrong.
///
/// # Errors
/// If the retrieval of users for a particular reaction fails (e.g. due to API errors), a log entry will be made, and the function will proceed with the other emojis.
///
/// # Example
/// ```rust
/// let exclude_reactions = vec![":thumbs up:".to_string()];
/// let reaction_data = reaction_users(&ctx, &message, &exclude_reactions).await;
///
/// match reaction_data {
///     Ok(data) => {
///         for (emoji, users) in data {
///             println!("Emoji: {}, Users: {:?}", emoji, users);
///         }
///     }
///     Err(e) => eprintln!("Failed to retrieve reaction data: {}", e),
/// }
/// ```
///
/// # Notes
/// - This function internally uses `get_filtered_users2` to fetch the users who reacted with a specific emoji.
/// - Any reactions listed in the `exclude_reactions` parameter are ignored and not included in the output.
///
/// # Dependencies
/// - The function depends on the `log` crate for error logging.
/// - The `reaction.reaction_type.to_string()` assumes that the `ReactionType` can be correctly converted to a `String`.
///
/// [`Context`]: https://docs.rs/serenity/*/serenity/model/prelude/struct.Context.html
/// [`Message`]: https://docs.rs/serenity/*/serenity/model/prelude/struct.Message.html
/// [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
/// [`User`]: https://docs.rs/serenity/*/serenity/model/user/struct.User.html
pub async fn to_reaction_map(
    ctx: &Context,
    message: &Message,
    exclude_reactions: &[String]
) -> Result<HashMap<String, Vec<User>>, serenity::Error> {
    let mut result: HashMap<String, Vec<User>> = HashMap::new();

    for reaction in &message.reactions {
        let reaction_emoji = reaction.reaction_type.to_string();

        // Skip excluded reactions
        if exclude_reactions.contains(&reaction_emoji) {
            continue;
        }

        match fetch_reaction_users(&ctx, &message, &reaction).await {
            Ok(users) => {
                result.insert(reaction_emoji, users);
            }
            Err(e) => {
                log::error!("Failed to get users to reaction {}: {}", reaction_emoji, e);
            }
        }
    }
    Ok(result)
}

/// Fetches and retrieves a list of users who reacted with a specific reaction to a given message.
///
/// This function uses the Discord API to iterate through all pages of users who reacted
/// with the specified reaction. It recursively fetches reaction users in batches (with a maximum
/// of 100 users per request) until all users have been retrieved.
///
/// # Parameters:
/// - `ctx`: A reference to the [`serenity::prelude::Context`] containing the bot's context,
///          including HTTP client and cache.
/// - `message`: A reference to the [`serenity::model::channel::Message`] that contains the specific
///              reaction to fetch users from.
/// - `reaction`: A reference to the [`serenity::model::channel::MessageReaction`] specifying the
///               reaction type to filter users by.
///
/// # Returns:
/// An asynchronous result containing a vector of [`serenity::model::user::User`] objects if successful.
/// If an error occurs while making a request to the Discord API, it returns a [`Result::Err`].
///
/// # Behaviour:
/// 1. Fetches users in batches (up to 100 users per request) using `reaction_users`.
/// 2. Continues to fetch users until all pages have been retrieved.
/// 3. Collects all users who reacted into a single `Vec<User>`.
///
/// # Errors:
/// - Returns an error if the bot encounters an API issue while fetching reaction users.
/// - Returns an error if the HTTP request fails or a network issue occurs.
///
/// # Example:
/// ```rust
/// # use serenity::model::user::User;
/// # use serenity::client::Context;
/// # use serenity::model::channel::{Message, MessageReaction};
/// # async fn example(ctx: &Context, message: &Message, reaction: &MessageReaction) -> Result<(), Box<dyn std::error::Error>> {
/// let users: Vec<User> = get_filtered_users2(ctx, message, reaction).await?;
/// println!("Number of users who reacted: {}", users.len());
/// # Ok(())
/// # }
/// ```
///
/// # Notes:
/// - Users are fetched and paginated through the Discord API using the `reaction_users` method.
async fn fetch_reaction_users(
    ctx: &Context,
    message: &Message,
    reaction: &serenity::model::channel::MessageReaction,
) -> Result<Vec<User>, serenity::Error> {
    // Fetch users who reacted with this specific reaction from Discord API
    let mut all_reaction_users = Vec::new();
    let mut after = None;

    // Discord API returns users in pages, so we need to fetch all pages
    loop {
        let users_page = message
            .channel_id
            .reaction_users(
                &ctx.http,
                message.id,
                reaction.reaction_type.clone(),
                Some(100), // Limit per request (max 100)
                after,
            )
            .await?;

        if users_page.is_empty() {
            break;
        }

        // Store the length before moving users_page
        let page_len = users_page.len();

        // Get the last user ID for pagination
        if let Some(last_user) = users_page.last() {
            after = Some(last_user.id);
        }

        all_reaction_users.extend(users_page);

        // If we got less than 100 users, we've reached the end
        if page_len < 100 {
            break;
        }
    }

    Ok(all_reaction_users)
}