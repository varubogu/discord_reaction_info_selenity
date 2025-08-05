use anyhow::Result;
use crate::Context;
use crate::services::reaction_users::process_reaction_members;
use crate::services::reaction_users::types::ReactionUsersParameter;

/// Handle the reaction members context menu command
#[poise::command(
    context_menu_command = "Get reaction members",
    slash_command,
    name_localized("ja", "リアクションユーザー集計"),
    description_localized("ja", "メッセージにリアクションしたユーザーを集計して表示します。"),
    ephemeral
)]
pub async fn get_reaction_members(
    ctx: Context<'_>,
    #[description = "The message ID or URL to fetch reactions from."]
    #[description_localized("ja", "リアクションを取得するメッセージのIDまたはURL")]
    message: poise::serenity_prelude::Message,
) -> Result<(), crate::Error> {

    ctx.defer().await?;

    let parameter = ReactionUsersParameter {
        message,
        is_reaction_grouping: false,
        is_author_include: false,
        is_show_count: false,
    };

    let response = process_reaction_members(
        ctx,
        &parameter
    ).await?;

    ctx.say(response.content).await?;

    Ok(())
}