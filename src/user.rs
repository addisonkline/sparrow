use chrono::{DateTime, Utc};

use crate::levels::UserLevel;

#[derive(Debug, Clone)]
pub struct User {
    // user metadata
    pub tag: String,
    pub alias: String,
    pub member_since: DateTime<Utc>,
    pub level: u8,
    // user state
    pub session_key: String,
    pub online: bool,
}

impl User {
    pub fn new(
        tag: String,
        alias: String,
        member_since: DateTime<Utc>,
        level: u8,
        session_key: String,
    ) -> Self {
        Self { 
            tag, 
            alias,
            level,
            member_since,
            session_key, 
            online: false 
        }
    }

    /// add a new user to the server
    pub fn create(tag: String, alias: String) -> Self {
        let now = Utc::now();
        Self::new(
            tag, 
            alias, 
            now, 
            UserLevel::Member.to_u8(),
            String::new()
        )
    }


}   