use serde_json::Value;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::server::Server;
use crate::codes::ReturnCodeType;
use crate::response::SparrowResponse;

pub async fn exec_disconnect_request(server: Arc<Mutex<Server>>, body: Value) -> Result<SparrowResponse> {
    let mut server_guard = server.lock().await;
    
    // get the user's session key from the body
    if !body.is_object() {
        return Ok(SparrowResponse::new(
            String::from("sparrow"),
            server_guard.version.clone(),
            String::from("disconnect"),
            ReturnCodeType::InvalidRequest as i32,
            String::from("invalid body"),
            Value::Null,
        ));
    }

    if !body.get("session_key").is_some() {
        return Ok(SparrowResponse::new(
            String::from("sparrow"),
            server_guard.version.clone(),
            String::from("disconnect"),
            ReturnCodeType::InvalidRequest as i32,
            String::from("missing 'session_key' in body"),
            Value::Null,
        ));
    }

    let session_key = body["session_key"].as_str().unwrap();

    // check if the user is connected and clone the user
    let user = server_guard.clients.iter()
        .find(|u| u.session_key == session_key)
        .cloned();
    
    if user.is_none() {
        return Ok(SparrowResponse::new(
            String::from("sparrow"),
            server_guard.version.clone(),
            String::from("disconnect"),
            ReturnCodeType::NotConnected as i32,
            String::from("user not connected"),
            Value::Null,
        ));
    }

    let user = user.unwrap();

    // log the user out
    server_guard.logout(user);

    // return the success response
    Ok(SparrowResponse::new(
        String::from("sparrow"),
        server_guard.version.clone(),
        String::from("disconnect"),
        ReturnCodeType::Success as i32,
        String::from("user disconnected"),
        Value::Null,
    ))
}