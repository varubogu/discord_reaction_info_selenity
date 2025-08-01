use anyhow::Result;
use serenity::{
    builder::{CreateInteractionResponse, CreateInteractionResponseFollowup},
    model::application::CommandInteraction,
    prelude::*,
};

use crate::services::reaction_users::{process_reaction_members, Parameter, Mode};
use crate::utils::parsers::{parse_user_mentions, parse_reactions, parse_message_identifier};

/// Handle the /rmem slash command
pub async fn handle_rmem_slash_command(
    ctx: &Context,
    command: &CommandInteraction,
) -> Result<()> {
    // Parse command options
    let mut message_param = String::new();
    let mut include_users = Vec::new();
    let mut exclude_users = Vec::new();
    let mut exclude_reactions = Vec::new();
    let mut mode = "reaction_members".to_string();

    for option in &command.data.options {
        match option.name.as_str() {
            "message" => {
                message_param = option.value.as_str().unwrap_or("").to_string();
            }
            "include_user" => {
                let users_str = option.value.as_str().unwrap_or("");
                include_users = parse_user_mentions(users_str);
            }
            "exclude_user" => {
                let users_str = option.value.as_str().unwrap_or("");
                exclude_users = parse_user_mentions(users_str);
            }
            "exclude_reaction" => {
                let reactions_str = option.value.as_str().unwrap_or("");
                exclude_reactions = parse_reactions(reactions_str);
            }
            "mode" => {
                mode = option.value.as_str().unwrap_or("reaction_members").to_string();
            }
            _ => {}
        }
    }

    // Defer the response to avoid 3-second timeout
    command
        .create_response(&ctx.http, CreateInteractionResponse::Defer(Default::default()))
        .await?;

    // Parse message URL or ID
    let guild_id = command.guild_id.unwrap_or_default();
    let channel_id = command.channel_id;

    let message_id = match parse_message_identifier(&message_param) {
        Ok(ids) => ids,
        Err(_) => {
            command
                .create_followup(&ctx.http, 
                    CreateInteractionResponseFollowup::new()
                        .content(format!("📝: {}\n\n⚠️ The message cannot be read.\n- The message does not exist.\n- You do not have permission to read the message.\n- The message has been deleted.", message_param))
                        .ephemeral(true)
                )
                .await?;
            return Ok(());
        }
    };

    // Fetch the message
    let message = match ctx.http.get_message(channel_id.into(), message_id.into()).await {
        Ok(msg) => msg,
        Err(_) => {
            command
                .create_followup(&ctx.http, 
                    CreateInteractionResponseFollowup::new()
                        .content(format!("📝: {}\n\n⚠️ The message cannot be read.\n- The message does not exist.\n- You do not have permission to read the message.\n- The message has been deleted.", message_param))
                        .ephemeral(true)
                )
                .await?;
            return Ok(());
        }
    };

    // Convert string mode to Mode enum
    let mode_enum = match mode.as_str() {
        "full" => Mode::Full,
        "reaction_count" => Mode::ReactionCount,
        "members" => Mode::Members,
        "members_author" => Mode::MembersAuthor,
        _ => Mode::ReactionMembers, // Default
    };

    // Create parameter struct
    let parameter = Parameter {
        include_users,
        exclude_users,
        exclude_reactions,
        mode: mode_enum,
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