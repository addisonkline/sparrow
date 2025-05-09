pub enum UserLevel {
    Member = 0,
    Moderator = 1,
    Admin = 2,
}

impl UserLevel {
    pub fn from_u8(level: u8) -> Option<Self> {
        match level {
            0 => Some(UserLevel::Member),
            1 => Some(UserLevel::Moderator),
            2 => Some(UserLevel::Admin),
            _ => None,
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
            UserLevel::Member => 0,
            UserLevel::Moderator => 1,
            UserLevel::Admin => 2,
        }
    }
}
