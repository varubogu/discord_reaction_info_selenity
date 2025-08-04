use std::env;
use dotenv::dotenv;
use poise::serenity_prelude::GatewayIntents;

mod events;
mod services;
mod utils;

pub(crate) type Data = ();
pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Context<'a> = poise::Context<'a, Data, Error>;


#[tokio::main]
async fn main() {
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

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands(),
            ..Default::default()
        })
        .setup(|_ctx, _ready, _framework| {
            Box::pin(async move {
                // Any setup code can go here
                poise::builtins::register_globally(_ctx, &_framework.options().commands).await?;
                Ok(())
            })
        })
        .build();

    // Create client
    let client = poise::serenity_prelude::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}

fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        events::interactions::command_interactions::slash::reaction_members::reaction_members(),
        events::interactions::command_interactions::contextmenu::reaction_users_context_menu::get_reaction_members(),
        events::interactions::command_interactions::contextmenu::reaction_grouping_users_context_menu::get_reaction_grouping_members(),
    ]
}

#[allow(dead_code)]
async fn error_handler(error: poise::FrameworkError<'_, Data, Error>) {
    println!("Oh no, we got an error: {:?}", error);
}