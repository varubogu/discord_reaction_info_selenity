use crate::services::reaction_users::process_reaction_members;
use crate::services::reaction_users::types::ReactionUsersParameter;
use anyhow::Result;
use serenity::all::CreateInteractionResponseMessage;
use serenity::{
    builder::{CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseFollowup},
    model::application::{CommandInteraction, CommandOptionType},
    prelude::*,
};

/// Create the /rmem slash command
pub fn create_command() -> CreateCommand {
    CreateCommand::new("rmem")
        .description("Get reaction members information from a message")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "message", "Message URL or Message ID")
                .required(true)
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::Boolean, "is_unique_users", "Whether to get unique users across all reactions")
                .required(false)
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "is_author_include", "Whether to include the author in the results")
                .required(false)
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "is_show_count", "Whether to show reaction counts")
                .required(false)
        )
        // .add_option(
        //     CreateCommandOption::new(CommandOptionType::String, "include_user", "Users to include (mention format)")
        //         .required(false)
        // )
        // .add_option(
        //     CreateCommandOption::new(CommandOptionType::String, "exclude_user", "Users to exclude (mention format)")
        //         .required(false)
        // )
        // .add_option(
        //     CreateCommandOption::new(CommandOptionType::String, "exclude_reaction", "Reactions to exclude")
        //         .required(false)
        // )
        // .add_option(
        //     CreateCommandOption::new(CommandOptionType::String, "mode", "Display mode")
        //         .required(false)
        //         .add_string_choice("reaction_members", "reaction_members")
        //         .add_string_choice("full", "full")
        //         .add_string_choice("reaction_count", "reaction_count")
        //         .add_string_choice("members", "members")
        //         .add_string_choice("members_author", "members_author")
        // )
}

/// Handle the /rmem slash command
pub async fn handle_rmem_slash_command(
    ctx: &Context,
    command: &CommandInteraction,
) -> Result<()> {

    // Acknowledge the interaction first
    command.create_response(&ctx.http, CreateInteractionResponse::Defer(
        CreateInteractionResponseMessage::new().ephemeral(true)
    )).await?;

    // Parse command options
    let mut message_param = String::new();
    // let mut include_users: Vec<UserId> = Vec::new();
    // let mut exclude_users: Vec<UserId> = Vec::new();
    // let mut exclude_reactions: Vec<String> = Vec::new();
    // let mut mode = "reaction_members".to_string(); // Default mode
    let mut is_author_include = false;
    let mut is_show_count = false;
    let mut is_unique_users = false;

    for option in &command.data.options {
        match option.name.as_str() {
            "message" => {
                message_param = option.value.as_str().unwrap_or("").to_string();
            }
            "is_unique_users" => {
                is_unique_users = option.value.as_bool().unwrap_or(false);
            }
            "is_author_include" => {
                is_author_include = option.value.as_bool().unwrap_or(false);
            }
            "is_show_count" => {
                is_show_count = option.value.as_bool().unwrap_or(false);
            }

            _ => {}
        }
    }

    let message = command.data.resolved.messages.values().next().cloned();
    let message = match message {
        Some(msg) => msg,
        None => {
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

    // Create parameter struct
    let parameter = ReactionUsersParameter {
        is_unique_users,
        is_author_include,
        is_show_count,
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

// async fn fetch_users(ctx: &&Context) {
//     // まとめてUser取得のために合成
//     let mut fetch_target_user_ids: Vec<u64> = Vec::new();
//     fetch_target_user_ids.append(include_users.clone().as_mut());
//     fetch_target_user_ids.append(exclude_users.clone().as_mut());
//
//     let distinct_user_ids: HashSet<u64> = fetch_target_user_ids
//         .iter()
//         .cloned()
//         .collect();
//
//     let fetch_target_users = distinct_user_ids
//         .iter()
//         .map(|id| UserId::new(id.clone()))
//         .collect::<Vec<UserId>>();
//
//     // APIでUserをまとめて取得
//     let fetch_users = fetch_discord_users(&ctx.http, &fetch_target_users).await;
//     let fetched_include_users = fetch_users
//         .iter()
//         .filter(|user| include_users.contains(user))
//         .cloned()
//         .collect::<Vec<User>>();
//
//     let fetched_exclude_users = fetch_users
//         .iter()
//         .filter(|user| exclude_users.contains(user))
//         .cloned()
//         .collect::<Vec<User>>();
// }