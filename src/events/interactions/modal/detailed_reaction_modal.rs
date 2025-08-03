use anyhow::Result;
use serenity::{
    builder::{CreateInteractionResponse, CreateInteractionResponseFollowup},
    model::application::ModalInteraction,
    prelude::*,
};
use serenity::all::{CreateInteractionResponseMessage, User};
use crate::services::r#mod::{process_reaction_members, ReactionUsersParameter, ReactionUsersMode};
use crate::utils::parsers::{parse_user_mentions, parse_reactions};

/// Handle the detailed reaction modal submission
pub async fn handle_detailed_reaction_modal(
    ctx: &Context,
    modal: &ModalInteraction,
) -> Result<()> {
    // Acknowledge the interaction first
    let response = CreateInteractionResponse::Defer(
        CreateInteractionResponseMessage::new().ephemeral(true)
    );

    modal.create_response(&ctx.http, response).await?;

    // Parse message information from modal custom_id
    let custom_id_parts: Vec<&str> = modal.data.custom_id.split(':').collect();
    if custom_id_parts.len() != 4 || custom_id_parts[0] != "detailed_reaction_modal" {
        modal
            .create_followup(&ctx.http, 
                CreateInteractionResponseFollowup::new()
                    .content("⚠️ Invalid modal data.")
                    .ephemeral(true)
            )
            .await?;
        return Ok(());
    }

    let guild_id_str = custom_id_parts[1];
    let channel_id: u64 = custom_id_parts[2].parse()?;
    let message_id: u64 = custom_id_parts[3].parse()?;

    // Fetch the message
    let message = ctx.http.get_message(channel_id.into(), message_id.into()).await?;

    // Parse modal input fields
    let mut include_users: Vec<User> = Vec::new();
    let mut exclude_users: Vec<User> = Vec::new();
    let mut exclude_reactions = Vec::new();
    let mut mode = ReactionUsersMode::ReactionMembers;

    // Extract values from modal components
    for action_row in &modal.data.components {
        for component in &action_row.components {
            match component {
                serenity::model::application::ActionRowComponent::InputText(input) => {
                    let custom_id = &input.custom_id;
                    let value = input.value.as_deref().unwrap_or("").trim();
                    
                    if value.is_empty() {
                        continue;
                    }

                    match custom_id.as_str() {
                        "include_users" => {
                            include_users = parse_user_mentions(value).await;
                        }
                        "exclude_users" => {
                            exclude_users = parse_user_mentions(value).await;
                        }
                        "exclude_reactions" => {
                            exclude_reactions = parse_reactions(value).await;
                        }
                        "mode" => {
                            mode = match value.to_lowercase().as_str() {
                                "reaction_members" => ReactionUsersMode::ReactionMembers,
                                "full" => ReactionUsersMode::Full,
                                "reaction_count" => ReactionUsersMode::ReactionCount,
                                "members" => ReactionUsersMode::Members,
                                "members_author" => ReactionUsersMode::MembersAuthor,
                                _ => ReactionUsersMode::ReactionMembers, // Default fallback
                            };
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    // Create parameter struct
    let parameter = ReactionUsersParameter {
        include_users,
        exclude_users,
        exclude_reactions,
        mode,
    };

    // Process reactions and generate response
    let response = process_reaction_members(
        ctx, 
        &message, 
        &parameter
    ).await?;
    
    modal
        .create_followup(&ctx.http, 
            CreateInteractionResponseFollowup::new()
                .content(response.content)
                .ephemeral(true)
        )
        .await?;

    Ok(())
}