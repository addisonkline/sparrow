use chrono::Utc;
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub sender_tag: String,
    pub content: String,
    pub timestamp: String,
}

impl Message {
    pub fn new(sender_tag: String, content: String, timestamp: String) -> Self {
        Self { sender_tag, content, timestamp }
    }

    pub fn create(sender_tag: String, content: String) -> Self {
        Self { sender_tag, content, timestamp: Utc::now().to_rfc3339() }
    }

    pub fn to_json(&self) -> Value {
        json!({
            "sender_tag": self.sender_tag,
            "content": self.content,
            "timestamp": self.timestamp
        })
    }
}