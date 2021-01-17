use anyhow::{bail, Result};
use base64::encode;
use serde::{de::DeserializeOwned, Deserialize};
use url::Url;

use crate::auth::load_token;

#[derive(Debug, Deserialize)]
struct Workspace {
    id: usize,
    name: String,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Deserialize)]
pub struct Project {
    pub id: usize,
    #[serde(rename = "wid")]
    pub workspace_id: usize,
    pub name: String,
}

fn make_request<T>(api_path: &str) -> Result<T>
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

pub fn workspaces_list() -> Result<Vec<Project>> {
    let workspaces: Vec<Workspace> = make_request("workspaces")?;

    let mut projects: Vec<Project> = vec![];

    for w in workspaces {
        let project_url = format!("workspaces/{}/projects", w.id);
        let mut workspace_projects: Vec<Project> = make_request(&project_url)?;
        projects.append(&mut workspace_projects);
    }

    Ok(projects.sort_by(|a, b| b.name.cmp(&a.name)))
}
