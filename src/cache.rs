use anyhow::{bail, Context, Result};
use directories::ProjectDirs;
use std::fs::{self, create_dir_all};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use crate::toggl::Project;

const CACHE_EXPIRY_SECS: u64 = 5 * 60;

fn project_cache() -> Result<PathBuf> {
    let project = ProjectDirs::from("net", "smoking-heaps", "togglctl").unwrap();
    let cache_dir = project.cache_dir();
    create_dir_all(&cache_dir)
        .with_context(|| format!("Failed to create cache dir {:?}", cache_dir))?;
    Ok(cache_dir.to_path_buf())
}

pub fn cache_projects(projects: &Vec<Project>) -> Result<()> {
    let cache_path = project_cache()?.join("projects.json");
    let data = serde_json::to_string(&projects)?;
    fs::write(cache_path, data)?;
    Ok(())
}

pub fn retrieve_projects_cache() -> Result<Vec<Project>> {
    let cache_path = project_cache()?.join("projects.json");
    let metadata = fs::metadata(&cache_path)?;

    if let Ok(time) = metadata.modified() {
        let now = SystemTime::now();
        // If in doubt invalidate the cache.
        let difference = now
            .duration_since(time)
            .unwrap_or(Duration::new(CACHE_EXPIRY_SECS + 1, 0));

        // We cache projects for five minutes.
        if difference.as_secs() > CACHE_EXPIRY_SECS {
            bail!("cache expired");
        }
    }

    let projects = fs::read_to_string(cache_path)?;
    Ok(serde_json::from_str(&projects)?)
}
