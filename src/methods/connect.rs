use serde_json::{Value, json};
use uuid::Uuid;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::server::Server;
use crate::codes::ReturnCodeType;
use crate::response::SparrowResponse;

/// given user's tag, return a response with a session key and mark the user as connected
pub async fn exec_connect_request(server: Arc<Mutex<Server>>, body: Value) -> Result<SparrowResponse> {
    let mut server_guard = server.lock().await;
    
    // get the user's tag from the body
    if !body.is_object() {
        return Ok(SparrowResponse::new(
            String::from("sparrow"),
            server_guard.version.clone(),
            String::from("connect"),
            ReturnCodeType::InvalidRequest as i32,
            String::from("invalid body"),
            Value::Null,
        ));
    }

    if !body.get("tag").is_some() {
        return Ok(SparrowResponse::new(
            String::from("sparrow"),
            server_guard.version.clone(),
            String::from("connect"),
            ReturnCodeType::InvalidRequest as i32,
            String::from("missing 'tag' in body"),
            Value::Null,
        ));
    }

    let tag = body["tag"].as_str().unwrap();

    // check if the user is a member of the server and clone the user
    let user = server_guard.members.iter()
        .find(|u| u.tag == tag)
        .cloned();
    
    if user.is_none() {
        return Ok(SparrowResponse::new(
            String::from("sparrow"),
            server_guard.version.clone(),
            String::from("connect"),
            ReturnCodeType::Unauthorized as i32,
            String::from("user not found"),
            Value::Null,
        ));
    }

    let user = user.unwrap();

    // check if the user is already connected
    if server_guard.clients.iter().find(|u| u.session_key == user.session_key).is_some() {
        return Ok(SparrowResponse::new(
            String::from("sparrow"),
            server_guard.version.clone(),
            String::from("connect"),
            ReturnCodeType::AlreadyConnected as i32,
            String::from("user already connected"),
            Value::Null,
        ));
    }

    // create a new session key
    let session_key = Uuid::new_v4().to_string();

    // log the user in
    server_guard.login(user);

    // return the session key
    Ok(SparrowResponse::new(
        String::from("sparrow"),
        server_guard.version.clone(),
        String::from("connect"),
        ReturnCodeType::Success as i32,
        String::from("user connected"),
        json!({
            "session_key": session_key,
        }),
    ))
}
