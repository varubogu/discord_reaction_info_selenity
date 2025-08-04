use poise::serenity_prelude::Message;

#[derive(Debug)]
pub struct ReactionUsersParameter {
    pub message: Message,
    pub is_reaction_grouping: bool,
    pub is_author_include: bool,
    pub is_show_count: bool,
}

#[derive(Debug)]
pub struct ReactionUsersResponse {
    pub content: String,
}