use tokio::{
    io::AsyncReadExt,
    net::TcpStream,
    sync::Mutex,
};
use anyhow::Result;
use serde_json::Value;
use std::sync::Arc;

use crate::request::SparrowRequest;
use crate::codes::ReturnCodeType;
use crate::methods::parse_request_by_method;
use crate::response::SparrowResponse;
use crate::server::Server;

pub async fn parse_tcp_stream(server: Arc<Mutex<Server>>, stream: &mut TcpStream) -> Result<SparrowResponse> {
    // read the message from the stream
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await?;
    let message = String::from_utf8(buffer.to_vec())?;

    // parse the message into an object
    if message.is_empty() {
        let server_guard = server.lock().await;
        return Ok(SparrowResponse::new(
            String::from("sparrow"),
            server_guard.version.clone(),
            String::from(""),
            ReturnCodeType::InvalidRequest as i32,
            String::from("empty message"),
            Value::Null,
        ));
    }

    // parse the message into a SparrowRequest
    match serde_json::from_str(&message) {  
        Ok(request) => {
            // generate the response object
            let response = parse_sparrow_request(server, request).await?;
            Ok(response)
        }
        Err(_) => {
            let server_guard = server.lock().await;
            return Ok(SparrowResponse::new(
                String::from("sparrow"),
                server_guard.version.clone(),
                String::from(""),
                ReturnCodeType::InvalidRequest as i32,
                String::from("invalid message"),
                Value::Null,
            ));
        }
    }
}

async fn parse_sparrow_request(server: Arc<Mutex<Server>>, request: SparrowRequest) -> Result<SparrowResponse> {
    let server_guard = server.lock().await;
    
    // ensure top-level params are valid
    if request.protocol != String::from("sparrow") {
        return Ok(SparrowResponse::new(
            String::from("sparrow"),
            server_guard.version.clone(),
            request.method,
            ReturnCodeType::InvalidRequest as i32,
            String::from("protocol name must be 'sparrow'"),
            Value::Null,
        ));
    }
    if request.version != server_guard.version {
        return Ok(SparrowResponse::new(
            String::from("sparrow"),
            server_guard.version.clone(),
            request.method,
            ReturnCodeType::InvalidRequest as i32,
            format!("invalid version: expected {}, got {}", server_guard.version, request.version),
            Value::Null,
        ));
    }
    
    // drop the guard before calling parse_request_by_method to avoid holding the lock
    drop(server_guard);
    
    // parse the request by method
    let response = parse_request_by_method(server, request).await?;    

    Ok(response)
}
