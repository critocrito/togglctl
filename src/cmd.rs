use anyhow::Result;
use std::fs;

use crate::auth::token_cache;
use crate::cache;
use crate::toggl;

pub fn set_auth(token: &str) -> Result<()> {
    let token_path = token_cache()?;
    fs::write(token_path, token)?;
    Ok(())
}

pub fn projects() -> Result<Vec<toggl::Project>> {
    let projects = match cache::retrieve_projects_cache() {
        Ok(projects) => projects,
        Err(_) => {
            let projects = toggl::list_projects()?;
            cache::cache_projects(&projects)?;
            projects
        }
    };

    Ok(projects)
}

pub fn start_timer(project: &str) -> Result<()> {
    let project: usize = project.parse()?;
    toggl::start_timer(project)?;

    Ok(())
}

pub fn stop_timer() -> Result<()> {
    toggl::stop_current_timer()?;

    Ok(())
}
