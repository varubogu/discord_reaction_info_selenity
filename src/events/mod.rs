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
use interactions::command_interactions::slash::rmem_slash;
use interactions::command_interactions::contextmenu::reaction_members_context_menu;
use interactions::command_interactions::contextmenu::reaction_users_context_menu;

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
    // Create commands using their respective modules
    let slash_command = rmem_slash::create_command();
    let reaction_members_context_menu_command = reaction_users_context_menu::create_command();
    let members_context_menu_command = reaction_members_context_menu::create_command();

    // Register all commands
    ctx.http.create_global_command(&slash_command).await?;
    ctx.http.create_global_command(&reaction_members_context_menu_command).await?;
    ctx.http.create_global_command(&members_context_menu_command).await?;
    info!("Commands registered successfully");
    Ok(())
}