use anyhow::Result;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::server::Server;
use crate::codes::ReturnCodeType;
use crate::message::Message;
use crate::response::SparrowResponse;

pub async fn exec_msg_send_request(server: Arc<Mutex<Server>>, body: Value) -> Result<SparrowResponse> {
    let server_guard = server.lock().await;
    
    // get the user's session key, length, and content from the body
    if !body.is_object() {
        return Ok(SparrowResponse::new(
            String::from("sparrow"),
            server_guard.version.clone(),
            String::from("msg_send"),
            ReturnCodeType::InvalidRequest as i32,
            String::from("invalid body"),
            Value::Null,
        ));
    }
    
    if !body.get("session_key").is_some() {
        return Ok(SparrowResponse::new(
            String::from("sparrow"),
            server_guard.version.clone(),
            String::from("msg_send"),
            ReturnCodeType::InvalidRequest as i32,
            String::from("missing 'session_key' in body"),
            Value::Null,
        ));
    }

    if !body.get("length").is_some() {
        return Ok(SparrowResponse::new(
            String::from("sparrow"),
            server_guard.version.clone(),
            String::from("msg_send"),
            ReturnCodeType::InvalidRequest as i32,
            String::from("missing 'length' in body"),
            Value::Null,
        ));
    }

    if !body.get("content").is_some() {
        return Ok(SparrowResponse::new(
            String::from("sparrow"),
            server_guard.version.clone(),
            String::from("msg_send"),
            ReturnCodeType::InvalidRequest as i32,
            String::from("missing 'content' in body"),
            Value::Null,
        ));
    }

    let session_key = body["session_key"].as_str().unwrap();
    let length = body["length"].as_i64().unwrap();
    let content = body["content"].as_str().unwrap();
    
    // check if the user is connected and clone the user
    let user = server_guard.clients.iter()
        .find(|u| u.session_key == session_key)
        .cloned();
    
    if user.is_none() {
        return Ok(SparrowResponse::new(
            String::from("sparrow"),
            server_guard.version.clone(),
            String::from("msg_send"),
            ReturnCodeType::Unauthorized as i32,
            String::from("not authorized"),
            Value::Null,
        ));
    }

    // ensure the length is within the bounds
    if length < 1 || length > 1024 {
        return Ok(SparrowResponse::new(
            String::from("sparrow"),
            server_guard.version.clone(),
            String::from("msg_send"),
            ReturnCodeType::InvalidRequest as i32,
            String::from("length must be between 1 and 1024"),
            Value::Null,
        ));
    }

    let user = user.unwrap();

    // create the message
    let message = Message::create(user.tag.clone(), content.to_string());
    
    // add the message to the server's messages
    server_guard.messages.lock().await.push(message);

    // return success
    Ok(SparrowResponse::new(
        String::from("sparrow"),
        server_guard.version.clone(),
        String::from("msg_send"),
        ReturnCodeType::Success as i32,
        String::from("message sent"),
        Value::Null,
    ))
}
