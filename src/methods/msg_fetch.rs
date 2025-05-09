use serde_json::Value;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::server::Server;
use crate::codes::ReturnCodeType;
use crate::message::Message;
use crate::response::SparrowResponse;

pub async fn exec_msg_fetch_request(server: Arc<Mutex<Server>>, body: Value) -> Result<SparrowResponse> {
    let server_guard = server.lock().await;
    
    // get the user's session key and number of messages to fetch from the body
    if !body.is_object() {
        return Ok(SparrowResponse::new(
            String::from("sparrow"),
            server_guard.version.clone(),
            String::from("msg_fetch"),
            ReturnCodeType::InvalidRequest as i32,
            String::from("invalid body"),
            Value::Null,
        ));
    }

    if !body.get("session_key").is_some() {
        return Ok(SparrowResponse::new(
            String::from("sparrow"),
            server_guard.version.clone(),
            String::from("msg_fetch"),
            ReturnCodeType::InvalidRequest as i32,
            String::from("missing 'session_key' in body"),
            Value::Null,
        ));
    }

    if !body.get("number").is_some() {
        return Ok(SparrowResponse::new(
            String::from("sparrow"),
            server_guard.version.clone(),
            String::from("msg_fetch"),
            ReturnCodeType::InvalidRequest as i32,
            String::from("missing 'number' in body"),
            Value::Null,
        ));
    }

    let session_key = body["session_key"].as_str().unwrap();
    let number = body["number"].as_i64().unwrap();

    // check if the user is connected
    let user = server_guard.clients.iter()
        .find(|u| u.session_key == session_key)
        .cloned();
    
    if user.is_none() {
        return Ok(SparrowResponse::new(
            String::from("sparrow"),
            server_guard.version.clone(),
            String::from("msg_fetch"),
            ReturnCodeType::Unauthorized as i32,
            String::from("not authorized"),
            Value::Null,
        ));
    }

    // ensure the number is within the bounds
    if number < 1 || number > 100 {
        return Ok(SparrowResponse::new(
            String::from("sparrow"),
            server_guard.version.clone(),
            String::from("msg_fetch"),
            ReturnCodeType::InvalidRequest as i32,
            String::from("number must be between 1 and 100"),
            Value::Null,
        ));
    }

    // fetch the messages
    let messages_lock = server_guard.messages.lock().await;
    let messages = messages_lock.iter().rev().take(number as usize).collect::<Vec<&Message>>();
    
    // return the messages
    Ok(SparrowResponse::new(
        String::from("sparrow"),
        server_guard.version.clone(),
        String::from("msg_fetch"),
        ReturnCodeType::Success as i32,
        String::from("messages fetched"),
        Value::Array(messages.iter().map(|m| m.to_json()).collect()),
    ))
}