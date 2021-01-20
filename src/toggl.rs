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

#[derive(Debug, Serialize, Deserialize)]
pub struct Timer {
    pub id: usize,
    #[serde(rename = "wid")]
    pub workspace_id: usize,
    pub start: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
struct TimerReq {
    pid: usize,
    start: DateTime<Utc>,
    created_with: String,
}

#[derive(Debug, Serialize)]
struct TimerReqEnvelope {
    time_entry: TimerReq,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct DataEnvelope<T>
where
    T: Serialize,
{
    data: T,
}

fn get_request<T>(api_path: &str) -> Result<Option<T>>
where
    T: DeserializeOwned,
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

    let data: Option<T> = match response?.into_json() {
        Ok(d) => Some(d),
        Err(_) => None,
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

fn put_request(api_path: &str) -> Result<()> {
    let token = load_token()?;
    let auth = encode(format!("{}:api_token", token));

    // The ending slash is significant, otherwise "v8" gets stripped from the
    // path.
    let endpoint = Url::parse("https://api.track.toggl.com/api/v8/")?;
    let url = endpoint.join(api_path)?;

    match ureq::put(url.as_str())
        .set("Authorization", format!("Basic {}", auth).as_str())
        .set("Content-Type", "application/json")
        // The API requires this to be set
        .set("Content-Length", "0")
        .call()
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

pub fn list_projects() -> Result<Vec<Project>> {
    let workspaces: Vec<Workspace> = get_request("workspaces")?.unwrap_or(vec![]);

    let mut projects: Vec<Project> = vec![];

    for w in workspaces {
        let project_url = format!("workspaces/{}/projects", w.id);
        if let Some(mut workspace_projects) = get_request::<Vec<Project>>(&project_url)? {
            projects.append(&mut workspace_projects);
        }
    }

    projects.sort_by(|a, b| b.name.cmp(&a.name));
    Ok(projects)
}

pub fn start_timer(project_id: usize) -> Result<()> {
    let req = TimerReqEnvelope {
        time_entry: TimerReq {
            pid: project_id,
            start: Utc::now(),
            created_with: "togglctl".to_string(),
        },
    };

    post_request("time_entries/start", req)?;

    Ok(())
}

pub fn get_running_timer() -> Result<Option<Timer>> {
    match get_request::<DataEnvelope<Timer>>("time_entries/current")? {
        Some(data) => Ok(Some(data.data)),
        None => Ok(None),
    }
}

pub fn stop_current_timer() -> Result<()> {
    if let Some(timer) = get_running_timer()? {
        put_request(format!("time_entries/{}/stop", timer.id).as_str())?;
    }

    Ok(())
}
