use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct SparrowRequest {
    pub protocol: String,
    pub version: String,
    pub method: String,
    pub body: Value,
}

impl SparrowRequest {
    pub fn new(protocol: String, version: String, method: String, body: Value) -> Self {
        Self { protocol, version, method, body }
    }
}