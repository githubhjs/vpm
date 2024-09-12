use anyhow::Result;
use directories::ProjectDirs;
use posthog_rs::{client, Event};
use std::fs;
use std::path::PathBuf;
use toml_edit::{DocumentMut, Item, Value, Table};
use uuid::Uuid;

pub fn send_event(command: &str) -> Result<()> {
    if get_analytics()? {
        let uuid = get_uuid()?;
        let client = client(std::env::var("POSTHOG_API_KEY").unwrap().as_str());
        let mut event = Event::new("user_action", &uuid);
        event.insert_prop("command", command)?;
        event.insert_prop("version", env!("CARGO_PKG_VERSION"))?;
        client.capture(event)?;
    }
    Ok(())
}

pub fn get_config_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "Instachip", "vpm")
        .map(|proj_dirs| proj_dirs.config_dir().to_path_buf())
        .map(|mut path| {
            path.push("config.toml");
            path
        })
}

pub fn create_config() -> Result<()> {
    let config_path = get_config_path().unwrap();
    fs::write(config_path.clone(), "").expect("Failed to create config.toml");
    let contents = fs::read_to_string(config_path.clone())?;
    let mut config_doc = contents.parse::<DocumentMut>().expect("Failed to parse config.toml");

    config_doc.insert("user", Item::Table(Table::new()));
    let user_table = config_doc["user"].as_table_mut().unwrap();
    user_table.insert("uuid", Item::Value(Value::from(Uuid::now_v7().to_string())));
    user_table.insert("os", Item::Value(Value::from(std::env::consts::OS)));
    user_table.insert("arch", Item::Value(Value::from(std::env::consts::ARCH)));

    config_doc.insert("tool", Item::Table(Table::new()));
    let tool_table = config_doc["tool"].as_table_mut().unwrap();
    tool_table.insert("version", Item::Value(Value::from(env!("CARGO_PKG_VERSION"))));

    config_doc.insert("options", Item::Table(Table::new()));
    let options_table = config_doc["options"].as_table_mut().unwrap();
    options_table.insert("analytics", Item::Value(Value::from(true)));

    fs::write(config_path, config_doc.to_string()).expect("Failed to write config.toml");
    Ok(())
}

fn get_uuid() -> Result<String> {
    let config_path = get_config_path().unwrap();
    if !config_path.exists() {
        create_config()?;
    }
    let contents = fs::read_to_string(config_path)?;
    let config = contents.parse::<DocumentMut>().expect("Failed to parse config.toml");
    Ok(config["uuid"].as_str().unwrap().to_string())
}

pub fn set_analytics(value: bool) -> Result<()> {
    let config_path = get_config_path().unwrap();
    let config = fs::read_to_string(config_path.clone())?;
    let mut config_doc = config.parse::<DocumentMut>().expect("Failed to parse config.toml");
    config_doc["options"]["analytics"] = Item::Value(Value::from(value));
    fs::write(config_path, config_doc.to_string()).expect("Failed to write config.toml");
    Ok(())
}

fn get_analytics() -> Result<bool> {
    let config_path = get_config_path().unwrap();
    let config = fs::read_to_string(config_path.clone())?;
    let config_doc = config.parse::<DocumentMut>().expect("Failed to parse config.toml");
    Ok(config_doc["options"]["analytics"].as_bool().unwrap())
}

pub fn set_version(version: &str) -> Result<()> {
    let config_path = get_config_path().unwrap();
    let config = fs::read_to_string(config_path.clone())?;
    let mut config_doc = config.parse::<DocumentMut>().expect("Failed to parse config.toml");
    config_doc["tool"]["version"] = Item::Value(Value::from(version));
    fs::write(config_path, config_doc.to_string()).expect("Failed to write config.toml");
    Ok(())
}   