use anyhow::{Context, Result};

pub async fn create_memo(output_dir: &str, verbose: bool, endpoint: Option<&str>) -> Result<()> {
    let endpoint_env =
        std::env::var("MEMO_DOMAIN").context("Failed to get MEMO_DOMAIN environment variable")?;

    let _ep = endpoint.unwrap_or(endpoint_env.as_str());

    // Implement another API endpoint here
    println!(
        "Getting data from other endpoint {} {}",
        output_dir, verbose
    );
    Ok(())
}
