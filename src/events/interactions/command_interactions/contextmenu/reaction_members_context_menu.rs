use anyhow::Result;
use serenity::{
    builder::{CreateInteractionResponse, CreateInteractionResponseFollowup},
    model::application::CommandInteraction,
    prelude::*,
};

use crate::services::reaction_users::{process_reaction_members, Parameter, Mode};

/// Handle the reaction members context menu command
pub async fn handle_reaction_members_context_menu(
    ctx: &Context,
    command: &CommandInteraction,
) -> Result<()> {
    // Get the target message from the context menu interaction
    let message = command.data.resolved.messages.values().next().cloned();

    // Defer the response to avoid 3-second timeout
    command
        .create_response(&ctx.http, CreateInteractionResponse::Defer(Default::default()))
        .await?;

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

    // Create parameter struct with mode=Members and empty arrays as specified
    let parameter = Parameter {
        include_users: Vec::new(),
        exclude_users: Vec::new(),
        exclude_reactions: Vec::new(),
        mode: Mode::Members,
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