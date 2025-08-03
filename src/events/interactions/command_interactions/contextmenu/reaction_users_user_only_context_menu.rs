use crate::services::reaction_users::process_reaction_members;
use crate::services::reaction_users::types::ReactionUsersParameter;
use anyhow::Result;
use serenity::all::CreateInteractionResponseMessage;
use serenity::{
    builder::{CreateCommand, CreateInteractionResponse, CreateInteractionResponseFollowup},
    model::application::{CommandInteraction, CommandType},
    prelude::*,
};

/// Create the Reaction Members context menu command
pub fn create_command() -> CreateCommand {
    CreateCommand::new("Reaction Members")
        .kind(CommandType::Message)
}

/// Handle the reaction users context menu command
pub async fn handle_reaction_users_context_menu(
    ctx: &Context,
    command: &CommandInteraction,
) -> Result<()> {
    // Acknowledge the interaction first
    let response = CreateInteractionResponse::Defer(
        CreateInteractionResponseMessage::new().ephemeral(true)
    );

    command.create_response(&ctx.http, response).await?;

    let message = command.data.resolved.messages.values().next().cloned();

    let message = match message {
        Some(msg) => msg,
        None => { // 処理中にメッセージが無くなった場合
            command
                .create_followup(&ctx.http,
                                 CreateInteractionResponseFollowup::new()
                                     .content("⚠️ Could not find the target message.")
                                     .ephemeral(true)
                )
                .await?;
            return Ok(());
        }
    };

    // Create a parameter struct with mode=Full and empty arrays as specified
    let parameter = ReactionUsersParameter {
        is_unique_users: false,
        is_author_include: true,
        is_show_count: true,
    };

    // Process reactions and generate response
    let response = process_reaction_members(
        ctx, 
        &message, 
        &parameter
    ).await?;
    
    command
        .create_followup(&ctx.http, 
            CreateInteractionResponseFollowup::new()
                .content(response.content)
                .ephemeral(true)
        )
        .await?;

    Ok(())
}