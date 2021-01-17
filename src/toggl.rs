use anyhow::{bail, Result};
use base64::encode;
use chrono::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

use crate::auth::load_token;

#[derive(Debug, Serialize, Deserialize)]
struct Workspace {
    id: usize,
    name: String,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Project {
    pub id: usize,
    #[serde(rename = "wid")]
    pub workspace_id: usize,
    pub name: String,
}

#[derive(Debug, Serialize)]
struct Timer {
    pid: usize,
    start: DateTime<Utc>,
    created_with: String,
}

#[derive(Debug, Serialize)]
struct TimerReq {
    time_entry: Timer,
}

fn get_request<T>(api_path: &str) -> Result<T>
where
    T: DeserializeOwned + Default,
{
    let token = load_token()?;
    let auth = encode(format!("{}:api_token", token));

    // The ending slash is significant, otherwise "v8" gets stripped from the
    // path.
    let endpoint = Url::parse("https://api.track.toggl.com/api/v8/")?;
    let url = endpoint.join(api_path)?;

    let response = match ureq::get(url.as_str())
        .set("Authorization", format!("Basic {}", auth).as_str())
        .set("Content-Type", "application/json")
        .call()
    {
        Ok(r) => Ok(r),
        Err(ureq::Error::Status(_, r)) => {
            let msg = format!("HTTP error {}: {}", r.status(), r.get_url());
            bail!(msg)
        }
        Err(e) => Err(e),
    };
    let data: T = match response?.into_json() {
        Ok(d) => d,
        Err(_) => Default::default(),
    };

    Ok(data)
}

fn post_request<T>(api_path: &str, body: T) -> Result<()>
where
    T: Serialize,
{
    let token = load_token()?;
    let auth = encode(format!("{}:api_token", token));

    // The ending slash is significant, otherwise "v8" gets stripped from the
    // path.
    let endpoint = Url::parse("https://api.track.toggl.com/api/v8/")?;
    let url = endpoint.join(api_path)?;

    match ureq::post(url.as_str())
        .set("Authorization", format!("Basic {}", auth).as_str())
        .send_json(ureq::json!(body))
    {
        Ok(r) => Ok(r),
        Err(ureq::Error::Status(_, r)) => {
            let msg = format!("HTTP error {}: {}", r.status(), r.get_url());
            bail!(msg)
        }
        Err(e) => Err(e),
    }?;

    Ok(())
}

pub fn workspaces_list() -> Result<Vec<Project>> {
    let workspaces: Vec<Workspace> = get_request("workspaces")?;

    let mut projects: Vec<Project> = vec![];

    for w in workspaces {
        let project_url = format!("workspaces/{}/projects", w.id);
        let mut workspace_projects: Vec<Project> = get_request(&project_url)?;
        projects.append(&mut workspace_projects);
    }

    projects.sort_by(|a, b| b.name.cmp(&a.name));
    Ok(projects)
}

pub fn start_timer(project_id: usize) -> Result<()> {
    let req = TimerReq {
        time_entry: Timer {
            pid: project_id,
            start: Utc::now(),
            created_with: "togglctl".to_string(),
        },
    };

    post_request("time_entries/start", req)?;

    Ok(())
}
