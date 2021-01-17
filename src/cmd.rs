use anyhow::Result;
use std::fs;

use crate::auth::token_cache;
use crate::toggl;

pub fn set_auth(token: &str) -> Result<()> {
    let token_path = token_cache()?;
    fs::write(token_path, token)?;
    Ok(())
}

pub fn projects() -> Result<()> {
    toggl::workspaces_list()?;
    Ok(())
}
