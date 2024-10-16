use crate::utils;
use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn get_memos(output_dir: &str, verbose: bool, endpoint: Option<&str>) -> Result<()> {
    let memoapi = std::env::var("MEMOAPI").context("Failed to get MEMOAPI environment variable")?;
    let endpoint_env =
        std::env::var("MEMO_DOMAIN").context("Failed to get MEMO_DOMAIN environment variable")?;

    let ep = endpoint.unwrap_or(endpoint_env.as_str());

    let client = Client::new();
    let resp = client
        .get(format!("{}/api/v1/memos", ep))
        .header("accept", "application/json")
        .header("Authorization", format!("Bearer {}", memoapi))
        .send()
        .await
        .context("Failed to send request")?
        .json::<Value>()
        .await
        .context("Failed to parse JSON response")?;

    let mut memo_mappings = HashMap::new();

    if let Some(memos) = resp["memos"].as_array() {
        for (index, memo) in memos.iter().enumerate() {
            let name = memo["name"].as_str().unwrap_or("");
            let uid = memo["uid"].as_str().unwrap_or("");
            let content = memo["content"].as_str().unwrap_or("");
            let creator = memo["creator"].as_str().unwrap_or("");
            let file_name = format!("memo_{}.md", index + 1);
            let create_time = memo["createTime"].as_str().unwrap_or("");
            let update_time = memo["updateTime"].as_str().unwrap_or("");
            let display_time = memo["displayTime"].as_str().unwrap_or("");
            let row_status = memo["rowStatus"].as_str().unwrap_or("");
            let pinned = memo["pinned"].as_bool().unwrap_or(false);
            utils::write_memo_file(output_dir, &file_name, content)
                .context("Failed to write memo file")?;
            let link = format!("{}/memo/{}", ep, uid);

            memo_mappings.insert(
                file_name,
                json!({
                    "name": name,
                    "uid": uid,
                    "content":content.chars().take(20).collect::<String>(),
                    "creator": creator,
                    "create_time": create_time,
                    "update_time": update_time,
                    "display_time": display_time,
                    "row_status": row_status,
                    "pinned": pinned,
                    "link":link
                }),
            );

            if verbose {
                println!(
                    "Processing memo {} {} {}",
                    content.chars().take(20).collect::<String>(),
                    creator,
                    link
                );
            }
        }
    }

    // Save memo mappings to a JSON file
    let mappings_file_path = Path::new(output_dir).join("memo_mappings.json");
    let mappings_json = serde_json::to_string_pretty(&memo_mappings)
        .context("Failed to serialize memo mappings")?;

    let mut file = File::create(&mappings_file_path)
        .await
        .context("Failed to create memo_mappings.json")?;
    file.write_all(mappings_json.as_bytes())
        .await
        .context("Failed to write memo_mappings.json")?;

    if verbose {
        println!("Memo mappings saved to: {:?}", mappings_file_path);
    }

    Ok(())
}
