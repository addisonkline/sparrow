use serde::{Deserialize, Serialize};
use serde_json::{Value, Result};

#[derive(Serialize, Deserialize)]
pub struct SparrowResponse {
    pub protocol: String,
    pub version: String,
    pub method: String,
    pub return_code: i32,
    pub message: String,
    pub body: Value,
}

impl SparrowResponse {
    pub fn new(protocol: String, version: String, method: String, return_code: i32, message: String, body: Value) -> Self {
        Self { protocol, version, method, return_code, message, body }
    }

    pub fn dump_json(&self) -> Result<String> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }
}