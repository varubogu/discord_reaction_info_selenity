use anyhow::Result;
use crate::Context;
use crate::services::reaction_users::process_reaction_members;
use crate::services::reaction_users::types::ReactionUsersParameter;

/// Handle the reaction members context menu command
#[poise::command(
    context_menu_command = "Get reaction-groping members",
    slash_command,
    ephemeral
)]
pub async fn get_reaction_grouping_members(
    ctx: Context<'_>,
    message: poise::serenity_prelude::Message,
) -> Result<(), crate::Error> {

    ctx.defer().await?;

    let parameter = ReactionUsersParameter {
        message,
        is_reaction_grouping: true,
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