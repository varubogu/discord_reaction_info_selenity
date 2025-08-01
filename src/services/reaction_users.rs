use anyhow::Result;
use serenity::{
    model::channel::Message,
    prelude::*,
};
use std::collections::HashSet;

pub struct Parameter {
    pub include_users: Vec<u64>,
    pub exclude_users: Vec<u64>,
    pub exclude_reactions: Vec<String>,
    pub mode: Mode,
}

pub struct Response {
    pub content: String,
}

pub enum Mode {
    ReactionMembers,
    Full,
    ReactionCount,
    Members,
    MembersAuthor,
}


/// Process reaction members and generate response based on mode
pub async fn process_reaction_members(
    ctx: &Context,
    message: &Message,
    parameter: &Parameter,
) -> Result<Response> {
    let message_url = format!("https://discord.com/channels/{}/{}/{}", 
        message.guild_id.map(|id| id.to_string()).unwrap_or_else(|| "@me".to_string()),
        message.channel_id,
        message.id
    );
    
    let author_mention = format!("<@{}>", message.author.id);
    
    if message.reactions.is_empty() {
        return Ok(Response {
            content: format!(
                "Information\n  📝: {}\n  🧔: {}\n\nReactions:\n  No one reacted.",
                message_url, author_mention
            ),
        });
    }

    match parameter.mode {
        Mode::ReactionMembers => {
            process_reaction_members_mode(ctx, message, &parameter.include_users, &parameter.exclude_users, &parameter.exclude_reactions, &message_url, &author_mention).await
        }
        Mode::Full => {
            process_full_mode(ctx, message, &parameter.include_users, &parameter.exclude_users, &parameter.exclude_reactions, &message_url, &author_mention).await
        }
        Mode::ReactionCount => {
            process_reaction_count_mode(ctx, message, &parameter.include_users, &parameter.exclude_users, &parameter.exclude_reactions, &message_url, &author_mention).await
        }
        Mode::Members => {
            process_members_mode(ctx, message, &parameter.include_users, &parameter.exclude_users, &parameter.exclude_reactions, &message_url, &author_mention).await
        }
        Mode::MembersAuthor => {
            process_members_author_mode(ctx, message, &parameter.include_users, &parameter.exclude_users, &parameter.exclude_reactions, &message_url, &author_mention).await
        }
    }
}

async fn process_reaction_members_mode(
    ctx: &Context,
    message: &Message,
    include_users: &[u64],
    exclude_users: &[u64],
    exclude_reactions: &[String],
    message_url: &str,
    author_mention: &str,
) -> Result<Response> {
    let mut result = format!("Information\n  📝: {}\n  🧔: {}\n\nReactions:\n", message_url, author_mention);
    
    for reaction in &message.reactions {
        let reaction_emoji = reaction.reaction_type.to_string();
        
        // Skip excluded reactions
        if exclude_reactions.contains(&reaction_emoji) {
            continue;
        }
        
        let users = get_filtered_users(ctx, message, reaction, include_users, exclude_users).await?;
        let user_mentions: Vec<String> = users.iter().map(|id| format!("<@{}>", id)).collect();
        
        result.push_str(&format!("  {}: {}\n", reaction_emoji, user_mentions.join(" ")));
    }
    
    Ok(Response {
        content: result,
    })
}

async fn process_full_mode(
    ctx: &Context,
    message: &Message,
    include_users: &[u64],
    exclude_users: &[u64],
    exclude_reactions: &[String],
    message_url: &str,
    author_mention: &str,
) -> Result<Response> {
    let mut result = format!("Information\n  📝: {}\n  🧔: {}\n\nReactions:\n", message_url, author_mention);
    
    for reaction in &message.reactions {
        let reaction_emoji = reaction.reaction_type.to_string();
        
        // Skip excluded reactions
        if exclude_reactions.contains(&reaction_emoji) {
            continue;
        }
        
        let users = get_filtered_users(ctx, message, reaction, include_users, exclude_users).await?;
        let user_mentions: Vec<String> = users.iter().map(|id| format!("<@{}>", id)).collect();
        
        result.push_str(&format!("  {}: {} {}\n", reaction_emoji, users.len(), user_mentions.join(" ")));
    }
    
    Ok(Response {
        content: result,
    })
}

async fn process_reaction_count_mode(
    ctx: &Context,
    message: &Message,
    include_users: &[u64],
    exclude_users: &[u64],
    exclude_reactions: &[String],
    message_url: &str,
    author_mention: &str,
) -> Result<Response> {
    let mut result = format!("Information\n  📝: {}\n  🧔: {}\n\nReactions:\n", message_url, author_mention);
    
    for reaction in &message.reactions {
        let reaction_emoji = reaction.reaction_type.to_string();
        
        // Skip excluded reactions
        if exclude_reactions.contains(&reaction_emoji) {
            continue;
        }
        
        let users = get_filtered_users(ctx, message, reaction, include_users, exclude_users).await?;
        
        result.push_str(&format!("  {}: {}\n", reaction_emoji, users.len()));
    }
    
    Ok(Response {
        content: result,
    })
}

async fn process_members_mode(
    ctx: &Context,
    message: &Message,
    include_users: &[u64],
    exclude_users: &[u64],
    exclude_reactions: &[String],
    message_url: &str,
    author_mention: &str,
) -> Result<Response> {
    let mut all_users = HashSet::new();
    
    for reaction in &message.reactions {
        let reaction_emoji = reaction.reaction_type.to_string();
        
        // Skip excluded reactions
        if exclude_reactions.contains(&reaction_emoji) {
            continue;
        }
        
        let users = get_filtered_users(ctx, message, reaction, include_users, exclude_users).await?;
        all_users.extend(users);
    }
    
    let user_mentions: Vec<String> = all_users.iter().map(|id| format!("<@{}>", id)).collect();
    
    Ok(Response {
        content: format!(
            "Information\n  📝: {}\n  🧔: {}\n\nmembers:\n  {}",
            message_url, author_mention, user_mentions.join(" ")
        ),
    })
}

async fn process_members_author_mode(
    ctx: &Context,
    message: &Message,
    include_users: &[u64],
    exclude_users: &[u64],
    exclude_reactions: &[String],
    message_url: &str,
    author_mention: &str,
) -> Result<Response> {
    let mut all_users = HashSet::new();
    
    // Add message author
    all_users.insert(message.author.id.get());
    
    for reaction in &message.reactions {
        let reaction_emoji = reaction.reaction_type.to_string();
        
        // Skip excluded reactions
        if exclude_reactions.contains(&reaction_emoji) {
            continue;
        }
        
        let users = get_filtered_users(ctx, message, reaction, include_users, exclude_users).await?;
        all_users.extend(users);
    }
    
    let user_mentions: Vec<String> = all_users.iter().map(|id| format!("<@{}>", id)).collect();
    
    Ok(Response {
        content: format!(
            "Information\n  📝: {}\n  🧔: {}\n\nmembers:\n  {}",
            message_url, author_mention, user_mentions.join(" ")
        ),
    })
}

async fn get_filtered_users(
    ctx: &Context,
    message: &Message,
    reaction: &serenity::model::channel::MessageReaction,
    include_users: &[u64],
    exclude_users: &[u64],
) -> Result<Vec<u64>> {
    // For now, return a placeholder implementation
    // TODO: Implement actual reaction user fetching from Discord API
    let mut users = Vec::new();
    
    // This is a simplified implementation - in reality we would need to:
    // 1. Fetch users who reacted with this specific reaction
    // 2. Apply include_users filter (if not empty, only include these users)
    // 3. Apply exclude_users filter (remove these users)
    
    // Placeholder: return empty list for now
    Ok(users)
}