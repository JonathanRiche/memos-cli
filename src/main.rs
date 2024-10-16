mod api;
mod cli;
mod utils;
use anyhow::Result;
use api::updateMemo::Memo;
use clap::Parser;
use dotenv::dotenv;
#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let cli = cli::Cli::parse();

    match cli.endpoint.as_str() {
        "getMemos" => {
            api::getMemos::get_memos(&cli.output_dir, cli.verbose, cli.memo_domain.as_deref())
                .await?
        }
        "createMemo" => {
            api::createMemo::create_memo(&cli.output_dir, cli.verbose, cli.memo_domain.as_deref())
                .await?
        }
        "updateMemo" => {
            if cli.bulk_update {
                println!("Updating all memos in content directory");
                api::updateMemo::update_all_memos(
                    &cli.output_dir,
                    cli.verbose,
                    cli.memo_domain.as_deref(),
                )
                .await?
            } else {
                println!("Updating single memo");
                if let Some(memo_name) = &cli.memo_name {
                    println!("Memo name: {}", memo_name);
                    let memo_update = Memo {
                        content: cli.content.clone(),
                        ..Default::default()
                    };
                    api::updateMemo::update_memo(memo_name, memo_update, cli.memo_domain.as_deref())
                        .await?
                } else {
                    println!("Error: Memo name or UID is required for single memo update");
                }
            }
        }
        // "other_endpoint" => {
        //     api::createMemo::get_other_data(&cli.output_dir, cli.verbose).await?;
        // }
        _ => println!("Unknown endpoint: {}", cli.endpoint),
    }

    Ok(())
}
