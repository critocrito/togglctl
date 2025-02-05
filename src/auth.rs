use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::{
    env,
    fs::{self, create_dir_all},
    path::PathBuf,
};

pub fn token_cache() -> Result<PathBuf> {
    let project = ProjectDirs::from("net", "smoking-heaps", "togglctl").unwrap();
    let cfg_dir = project.config_dir();
    create_dir_all(&cfg_dir)
        .with_context(|| format!("Failed to create config dir {:?}", cfg_dir))?;
    Ok(cfg_dir.join("api_token"))
}

pub fn load_token() -> Result<String> {
    match env::var("TOGGLTRACK_ACCESS_TOKEN") {
        Ok(token) => Ok(token),
        Err(_) => {
            let token_path = token_cache()?;
            let token = fs::read_to_string(token_path)
                .context("Failed to load API token. Maybe try: togglctl set-auth <token>")?;
            Ok(token)
        }
    }
}
