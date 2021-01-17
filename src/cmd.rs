use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs::{self, create_dir_all};
use std::path::PathBuf;

fn token_cache() -> Result<PathBuf> {
    let project = ProjectDirs::from("net", "smoking-heaps", "togglctl").unwrap();
    let cfg_dir = project.config_dir();
    create_dir_all(&cfg_dir)
        .with_context(|| format!("Failed to create config dir {:?}", cfg_dir))?;
    Ok(cfg_dir.join("api_token"))
}

fn load_token() -> Result<String> {
    let token_path = token_cache()?;
    let token = fs::read_to_string(token_path)
        .context("Failed to load API token. Maybe try: togglctl set-auth <token>")?;
    Ok(token)
}

pub fn set_auth(token: &str) -> Result<()> {
    let token_path = token_cache()?;
    fs::write(token_path, token)?;
    Ok(())
}
