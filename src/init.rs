use anyhow::Result;
use std::fs;
use serde::Serialize;
use directories::ProjectDirs;
use toml;
use uuid;

#[derive(Serialize)]
struct Config {
    host: String,
    port: u16,
    name: String,
    blurb: String,
    key: String,
}

/// initialize a new server given a name and blurb
pub async fn init_server(name: String, blurb: String) -> Result<()> {
    // create the server directory
    let sparrow_server_dirs = ProjectDirs::from("com", "sparrow", "sparrow-server");
    let server_dir = sparrow_server_dirs.unwrap().data_dir().join("servers").join(&name);
    if !server_dir.exists() {
        fs::create_dir_all(&server_dir)?;
    } else {
        anyhow::bail!("server with name {} already exists", name);
    }

    // create the server config file and write the config
    let config_file = server_dir.join("config.toml");
    let config = Config {
        host: "0.0.0.0".to_string(),
        port: 8848,
        name: name.clone(),
        blurb: blurb.clone(),
        key: uuid::Uuid::new_v4().to_string(),
    };
    let config_str = toml::to_string(&config)?;
    fs::write(config_file, config_str)?;

    // create the server data directory
    let server_data_dir = server_dir.join("data");
    fs::create_dir_all(&server_data_dir)?;

    // create tables stored in the server data directory

    Ok(())
}