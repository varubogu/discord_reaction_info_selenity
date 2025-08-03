use anyhow::Result;
use serenity::{
    builder::{CreateCommand, CreateInteractionResponse, CreateModal, CreateInputText},
    model::application::{CommandInteraction, CommandType, InputTextStyle},
    prelude::*,
};
use serenity::all::{CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption};

/// Create the Detailed Members context menu command
pub fn create_command() -> CreateCommand {
    CreateCommand::new("Detailed Members")
        .kind(CommandType::Message)
}

/// Handle the detailed reaction members context menu command
pub async fn handle_detailed_reaction_members_context_menu(
    ctx: &Context,
    command: &CommandInteraction,
) -> Result<()> {
    // Get the target message from the context menu interaction
    let message = command.data.resolved.messages.values().next();
    
    let message = match message {
        Some(msg) => msg,
        None => {
            let response = serenity::builder::CreateInteractionResponse::Message(
                serenity::all::CreateInteractionResponseMessage::new()
                    .content("⚠️ Could not find the target message.")
                    .ephemeral(true)
            );
            command.create_response(&ctx.http, response).await?;
            return Ok(());
        }
    };

    // Include message info in modal custom_id for later retrieval
    let modal_id = format!("detailed_reaction_modal:{}:{}:{}", 
        message.guild_id.map(|id| id.to_string()).unwrap_or_else(|| "@me".to_string()),
        message.channel_id,
        message.id
    );

    // Create modal with input fields for detailed parameters
    let modal = CreateModal::new(modal_id, "Reaction Members - Detailed Options")
        .components(vec![
            serenity::builder::CreateActionRow::
            // serenity::builder::CreateActionRow::InputText(
            //     CreateInputText::new(InputTextStyle::Short, "include_users", "Include Users")
            //         .placeholder("@user1 @user2 or user IDs (optional)")
            //         .required(false)
            //         .max_length(1000)
            // ),
            // serenity::builder::CreateActionRow::InputText(
            //     CreateInputText::new(InputTextStyle::Short, "exclude_users", "Exclude Users")
            //         .placeholder("@user1 @user2 or user IDs (optional)")
            //         .required(false)
            //         .max_length(1000)
            // ),
            // serenity::builder::CreateActionRow::InputText(
            //     CreateInputText::new(InputTextStyle::Short, "exclude_reactions", "Exclude Reactions")
            //         .placeholder("👍 ❤️ 😂 (optional)")
            //         .required(false)
            //         .max_length(500)
            // ),
            // serenity::builder::CreateActionRow::InputText(
            //     CreateInputText::new(InputTextStyle::Short, "mode", "Mode")
            //         .placeholder("reaction_members, full, reaction_count, members, members_author")
            //         .value("reaction_members")
            //         .required(false)
            //         .max_length(50)
            // ),
            // serenity::builder::CreateActionRow::SelectMenu(
            //     CreateSelectMenu::new("Mode", CreateSelectMenuKind::String {
            //         options: vec![
            //             CreateSelectMenuOption::new("reaction_members", "Reaction Members"),
            //             CreateSelectMenuOption::new("full", "Full"),
            //             CreateSelectMenuOption::new("reaction_count", "Reaction Count"),
            //             CreateSelectMenuOption::new("members", "Members"),
            //             CreateSelectMenuOption::new("members_author", "Members Author")
            //         ]
            //     })
            // ),
        ]);

    let response = CreateInteractionResponse::Modal(modal);
    command.create_response(&ctx.http, response).await?;

    Ok(())
}