use std::path::PathBuf;

use anyhow::{Ok, Result};

mod cli;
mod scraper;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::main();
    let tmp = args.tmp_dir.unwrap_or_else(|| PathBuf::from("./tmp"));
    tokio::fs::create_dir_all(&tmp).await?;
    // println!("Postgres URL: {}", args.postgres_url);

    // Download all files first
    scraper::download_all(tmp).await?;

    Ok(())
}
