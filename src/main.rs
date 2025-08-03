use std::env;
use anyhow::Result;
use dotenv::dotenv;
use serenity::all::GatewayIntents;
use serenity::Client;
use tracing::error;
use crate::events::Handler;

mod events;
mod services;
mod utils;

#[tokio::main]
async fn main() -> Result<()>
{
    // Load environment variables
    dotenv().ok();
    
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get Discord token from environment
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected DISCORD_TOKEN in environment");

    // Set gateway intents
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::MESSAGE_CONTENT;

    // Create client
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Start the client
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }

    Ok(())
}