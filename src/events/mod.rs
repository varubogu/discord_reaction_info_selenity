use anyhow::Result;
use serenity::{
    async_trait,
    model::{
        application::Interaction,
        gateway::Ready,
    },
    prelude::*,
};
use tracing::{error, info};

pub mod interactions;

use interactions::command_interactions::handle_command_interaction;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        // Register global commands
        if let Err(why) = register_commands(&ctx).await {
            error!("Failed to register commands: {:?}", why);
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            if let Err(why) = handle_command_interaction(&ctx, &command).await {
                error!("Error handling command interaction: {:?}", why);
            }
        }
    }
}

async fn register_commands(ctx: &Context) -> Result<()> {
    use serenity::builder::{CreateCommand, CreateCommandOption};
    use serenity::model::application::CommandOptionType;

    let command = CreateCommand::new("rmem")
        .description("Get reaction members information from a message")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "message", "Message URL or Message ID")
                .required(true)
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "include_user", "Users to include (mention format)")
                .required(false)
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "exclude_user", "Users to exclude (mention format)")
                .required(false)
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "exclude_reaction", "Reactions to exclude")
                .required(false)
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "mode", "Display mode")
                .required(false)
                .add_string_choice("reaction_members", "reaction_members")
                .add_string_choice("full", "full")
                .add_string_choice("reaction_count", "reaction_count")
                .add_string_choice("members", "members")
                .add_string_choice("members_author", "members_author")
        );

    ctx.http.create_global_command(&command).await?;
    info!("Commands registered successfully");
    Ok(())
}