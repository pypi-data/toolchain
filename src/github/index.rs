use anyhow::Context;
use crate::github::{get_client, GithubError};
use crate::repository::index::RepositoryIndex;
use ureq::Agent;

pub fn get_repository_index(token: &str, name: &str, client: Option<Agent>) -> Result<RepositoryIndex, GithubError> {
    let client = client.unwrap_or_else(|| get_client());

    let response = client
        .get(&format!("https://api.github.com/repos/pypi-data/{name}/contents/index.json"))
        .set("Authorization", &format!("bearer {token}"))
        .set("X-GitHub-Api-Version", "2022-11-28")
        .set("Accept", "application/vnd.github.raw").call()?;

    Ok(serde_json::from_str(&response.into_string()?).with_context(|| format!("Error getting index content for {name}"))?)
}
