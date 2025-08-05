use crate::services::reaction_users::process_reaction_members;
use crate::services::reaction_users::types::ReactionUsersParameter;
use anyhow::Result;
use crate::Context;

/// Handle the /rmem slash command
#[poise::command(
    slash_command,
    name_localized("ja", "リアクションユーザー集計"),
    description_localized("ja", "メッセージにリアクションしたユーザーを集計して表示します。"),
    ephemeral
)]
pub async fn reaction_members(
    ctx: Context<'_>,

    #[description = "The message ID or URL to fetch reactions from."]
    #[description_localized("ja", "リアクションを取得するメッセージのIDまたはURL")]
    message: poise::serenity_prelude::Message,

    #[description = "Whether to include the message author in the results."]
    #[description_localized("ja", "メッセージ送信者を含めるかどうか")]
    is_author_include: Option<bool>,

    #[description = "Whether to include the count of reactions in the results."]
    #[description_localized("ja", "リアクションの件数表示を含めるかどうか")]
    is_show_count: Option<bool>,

    #[description = "True: Counts users for each reaction. False: Counts users by combining all reactions."]
    #[description_localized("ja", "True: リアクションごとにユーザーを集計します。 False: 全てのリアクションを合算してユーザーを集計します。")]
    is_reaction_grouping: Option<bool>,
) -> Result<(), crate::Error> {

    // Acknowledge the interaction first
    ctx.defer().await?;

    // Create parameter struct
    let parameter = ReactionUsersParameter {
        message,
        is_reaction_grouping: is_reaction_grouping.unwrap_or(false),
        is_author_include: is_author_include.unwrap_or(false),
        is_show_count: is_show_count.unwrap_or(false),
    };

    // Process reactions and generate a response
    let response = process_reaction_members(
        ctx,
        &parameter
    ).await;

    match response {
        Ok(response) => {
            // If processing was successful, send the response content
            ctx.say(response.content).await?;
        },
        Err(e) => {
            // If there was an error, send an error message
            ctx.say(format!("⚠️ Error: {}", e)).await?;
        }
    }

    Ok(())
}

