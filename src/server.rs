use std::sync::Arc;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
    sync::{broadcast, Mutex, mpsc},
    signal,
};
use anyhow::Result;
use uuid::Uuid;

use crate::message::Message;
use crate::user::User;
use crate::parser::parse_tcp_stream;

pub struct Server {
    // server metadata
    pub name: String,
    pub blurb: String,
    pub key: Uuid,
    pub version: String,
    // server data
    pub members: Vec<User>,
    pub clients: Vec<User>,
    pub messages: Arc<Mutex<Vec<Message>>>,
}

impl Server {
    pub fn new(
        name: String,
        blurb: String,
        key: Uuid,
        version: String,
        members: Vec<User>,
        clients: Vec<User>,
        messages: Arc<Mutex<Vec<Message>>>,
    ) -> Self {
        Self { 
            name,
            blurb,
            key,
            version,
            members,
            clients,
            messages,
        }
    }

    /// initialize the server for the first time
    pub fn create(
        name: String,
        blurb: String,
        version: String,
    ) -> Result<Self> {
        let key = Uuid::new_v4();
        let members = Vec::new();
        let clients = Vec::new();
        let messages = Arc::new(Mutex::new(Vec::new()));
        Ok(Self::new(name, blurb, key, version, members, clients, messages))
    }

    /// add a new user to the server
    pub fn add_user(&mut self, user: User) {
        self.members.push(user);
    }

    /// log a user into the server
    pub fn login(&mut self, user: User) {
        self.clients.push(user);
    }

    /// log a user out of the server
    pub fn logout(&mut self, user: User) {
        if let Some(pos) = self.clients.iter().position(|u| u.session_key == user.session_key) {
            self.clients.remove(pos);
        }
    }
}

// Thread-safe wrapper for Server
pub struct SharedServer {
    server: Arc<Mutex<Server>>,
    shutdown_tx: mpsc::Sender<()>,
    shutdown_rx: mpsc::Receiver<()>,
}

impl SharedServer {
    pub fn new(server: Server) -> Self {
        let (shutdown_tx, shutdown_rx) = mpsc::channel(1);
        Self {
            server: Arc::new(Mutex::new(server)),
            shutdown_tx,
            shutdown_rx,
        }
    }

    pub async fn add_user(&self, user: User) {
        let mut server = self.server.lock().await;
        server.add_user(user);
    }

    pub async fn login(&self, user: User) {
        let mut server = self.server.lock().await;
        server.login(user);
    }

    pub async fn logout(&self, user: User) {
        let mut server = self.server.lock().await;
        server.logout(user);
    }

    pub async fn shutdown(&self) -> Result<()> {
        println!("[sparrow-server] shutting down gracefully...");
        println!("[sparrow-server] dumping messages...");
        
        // Get a reference to messages before sending shutdown signal
        let messages = {
            let server = self.server.lock().await;
            server.messages.clone()
        };
        
        // Send shutdown signal
        let _ = self.shutdown_tx.send(()).await;
        
        // Dump messages
        dump(messages).await;
        
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        let listener = TcpListener::bind("0.0.0.0:8848").await?;
        let (_tx, _rx) = broadcast::channel::<String>(100);

        println!("[sparrow-server] chat server running on port 8848");

        // spawn a task to handle shutdown signals
        let shutdown_tx = self.shutdown_tx.clone();
        tokio::spawn(async move {
            tokio::select! {
                _ = signal::ctrl_c() => {
                    println!("\n[sparrow-server] received shutdown signal");
                    let _ = shutdown_tx.send(()).await;
                }
            }
        });

        loop {
            tokio::select! {
                // handle new connections
                result = listener.accept() => {
                    let (socket, addr) = result?;
                    println!("[sparrow-server] {addr} connected");
                    let who = addr.to_string();
                    
                    // clone the Arc<Mutex<Server>> for the new task
                    let server = self.server.clone();
                    
                    tokio::spawn(async move {
                        if let Err(e) = handle_client(server, socket).await {
                            eprintln!("[sparrow-server] {who} error: {e}");
                        }
                    });
                }
                // handle shutdown signal
                _ = self.shutdown_rx.recv() => {
                    self.shutdown().await?;
                }
            }
        }
    }
}

async fn handle_client(server: Arc<Mutex<Server>>, mut socket: TcpStream) -> Result<()> {
    // parse the tcp stream
    let response = parse_tcp_stream(server, &mut socket).await?;

    // send the response to the client
    socket.write_all(response.dump_json()?.as_bytes()).await?;

    Ok(())
}

pub async fn dump(messages: Arc<Mutex<Vec<Message>>>) {
    let messages = messages.lock().await;
    for message in messages.iter() {
        println!("{:?}", message);
    }
}