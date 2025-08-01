use anyhow::Result;
use serenity::{
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse},
    model::application::CommandInteraction,
    prelude::*,
};
use tracing::error;

pub mod slash;
pub mod contextmenu;

use slash::rmem_slash::handle_rmem_slash_command;

/// Handle command interactions (slash commands and context menus)
pub async fn handle_command_interaction(
    ctx: &Context,
    command: &CommandInteraction,
) -> Result<()> {
    match command.data.name.as_str() {
        "rmem" => {
            // Acknowledge the interaction first
            let response = CreateInteractionResponse::Defer(
                CreateInteractionResponseMessage::new().ephemeral(true)
            );
            
            command.create_response(&ctx.http, response).await?;

            // Process the command
            if let Err(why) = handle_rmem_slash_command(ctx, command).await {
                error!("Error processing rmem command: {:?}", why);
                
                // Send error response
                command
                    .edit_response(&ctx.http, 
                        EditInteractionResponse::new()
                            .content("An error occurred while processing the command.")
                    )
                    .await?;
            }
        }
        _ => {
            let response = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("Unknown command")
                    .ephemeral(true)
            );
            command.create_response(&ctx.http, response).await?;
        }
    }

    Ok(())
}