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
use contextmenu::reaction_users_context_menu::handle_reaction_users_context_menu;
use contextmenu::reaction_members_context_menu::handle_reaction_members_context_menu;

/// Handle command interactions (slash commands and context menus)
pub async fn handle_command_interaction(
    ctx: &Context,
    command: &CommandInteraction,
) -> Result<()> {
    match command.data.name.as_str() {
        "rmem" => {
            // Process the slash command
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
        "Reaction Members" => {
            // Process the context menu command
            if let Err(why) = handle_reaction_users_context_menu(ctx, command).await {
                error!("Error processing Reaction Members context menu: {:?}", why);
                
                // Send error response
                command
                    .edit_response(&ctx.http, 
                        EditInteractionResponse::new()
                            .content("An error occurred while processing the command.")
                    )
                    .await?;
            }
        }
        "Members" => {
            // Process the Members context menu command
            if let Err(why) = handle_reaction_members_context_menu(ctx, command).await {
                error!("Error processing Members context menu: {:?}", why);
                
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