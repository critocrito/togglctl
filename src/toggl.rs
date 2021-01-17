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

fn make_request<T>(api_path: &str) -> Result<T>
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
    let data: T = response?.into_json()?;

    Ok(data)
}

pub fn workspaces_list() -> Result<Vec<Workspace>> {
    let body: Vec<Workspace> = make_request("workspaces")?;

    Ok(body)
}
