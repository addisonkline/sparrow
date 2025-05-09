pub mod connect;
pub mod disconnect;
pub mod msg_fetch;
pub mod msg_send;

use anyhow::Result;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::server::Server;
use crate::request::SparrowRequest;
use crate::codes::ReturnCodeType;
use crate::response::SparrowResponse;

use crate::methods::connect::exec_connect_request;
use crate::methods::disconnect::exec_disconnect_request;
use crate::methods::msg_fetch::exec_msg_fetch_request;
use crate::methods::msg_send::exec_msg_send_request;

pub async fn parse_request_by_method(server: Arc<Mutex<Server>>, request: SparrowRequest) -> Result<SparrowResponse> {
    // parse the request
    let response = match request.method.as_str() {
        "connect" => exec_connect_request(server.clone(), request.body).await?,
        "disconnect" => exec_disconnect_request(server.clone(), request.body).await?,
        "msg_fetch" => exec_msg_fetch_request(server.clone(), request.body).await?,
        "msg_send" => exec_msg_send_request(server.clone(), request.body).await?,
        _ => {
            let server_guard = server.lock().await;
            SparrowResponse::new(
                String::from("sparrow"),
                server_guard.version.clone(),
                request.method,
                ReturnCodeType::InvalidRequest as i32,
                String::from("invalid method"),
                Value::Null,
            )
        },
    };

    Ok(response)
}

