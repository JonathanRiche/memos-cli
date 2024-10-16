use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use tokio::fs;
#[derive(Debug, Serialize, Deserialize)]
struct Resource {
    name: Option<String>,
    uid: Option<String>,
    filename: Option<String>,
    content: Option<String>,
    external_link: Option<String>,
    #[serde(rename = "type")]
    resource_type: Option<String>,
    size: Option<String>,
    memo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Relation {
    memo: Option<String>,
    related_memo: Option<String>,
    #[serde(rename = "type")]
    relation_type: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Memo {
    pub uid: Option<String>,
    pub row_status: Option<String>,
    pub creator: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub display_time: Option<String>,
    pub content: Option<String>,
    pub visibility: Option<String>,
    pub tags: Option<Vec<String>>,
    pub pinned: Option<bool>,
    pub resources: Option<Vec<Resource>>,
    pub relations: Option<Vec<Relation>>,
    pub snippet: Option<String>,
}

#[derive(Debug, Serialize)]
struct MemoUpdateRequest {
    content: String,
    pinned: bool,
}

pub async fn update_memo(
    memo_name: &str,
    updated_memo: Memo,
    endpoint: Option<&str>,
) -> Result<()> {
    let memoapi = env::var("MEMOAPI")?;
    let endpoint_env =
        std::env::var("MEMO_DOMAIN").context("Failed to get MEMO_DOMAIN environment variable")?;

    let ep = endpoint.unwrap_or(endpoint_env.as_str());

    let client = Client::new();
    let update_request = MemoUpdateRequest {
        content: updated_memo.content.unwrap(),
        pinned: updated_memo.pinned.unwrap_or(false),
    };
    let response = client
        .patch(format!("{}/api/v1/{}", ep, memo_name))
        .header("accept", "application/json")
        .header("Authorization", format!("Bearer {}", memoapi))
        .json(&update_request)
        .send()
        .await
        .context("Failed to update memo")?;
    if response.status().is_success() {
        println!("Memo updated successfully");

        Ok(())
    } else {
        println!("Failed to update memo");
        let error = response
            .text()
            .await
            .context("Failed to get error response")?;
        Err(anyhow::anyhow!(error))
    }
}

pub async fn update_all_memos(
    content_dir: &str,
    verbose: bool,
    endpoint: Option<&str>,
) -> Result<()> {
    let mappings_path = Path::new(content_dir).join("memo_mappings.json");
    let mappings_content = fs::read_to_string(&mappings_path)
        .await
        .context("Failed to read memo_mappings.json")?;
    let memo_mappings: HashMap<String, Value> =
        serde_json::from_str(&mappings_content).context("Failed to parse memo_mappings.json")?;
    let endpoint_env =
        std::env::var("MEMO_DOMAIN").context("Failed to get MEMO_DOMAIN environment variable")?;

    let ep = endpoint.unwrap_or(endpoint_env.as_str());

    println!("Updating {} memos", memo_mappings.len());
    for (file_name, metadata) in memo_mappings.iter() {
        let uid = metadata["name"]
            .as_str()
            .context("Failed to get UID from metadata")?;
        let file_path = Path::new(content_dir).join(file_name);

        println!("Updating {}", file_name);
        if file_path.exists() {
            let content = fs::read_to_string(&file_path)
                .await
                .context(format!("Failed to read file: {}", file_name))?;

            let updated_memo = Memo {
                content: Some(content),
                pinned: metadata["pinned"].as_bool(),
                ..Default::default()
            };

            match update_memo(uid, updated_memo, Some(ep)).await {
                Ok(_) => {
                    if verbose {
                        println!("Updated memo: {} ({})", file_name, uid)
                    }
                }
                Err(e) => println!(
                    "Failed to update memo: {} ({}). Error: {}",
                    file_name, uid, e
                ),
            }
        } else {
            println!("File not found, skipping update: {}", file_name);
        }
    }

    Ok(())
}
