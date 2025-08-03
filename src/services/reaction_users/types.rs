#[derive(Debug)]
pub struct ReactionUsersParameter {
    pub is_unique_users: bool,
    pub is_author_include: bool,
    pub is_show_count: bool,
}

#[derive(Debug)]
pub struct ReactionUsersResponse {
    pub content: String,
}