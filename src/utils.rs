use anyhow::{Context, Result};
use regex::Regex;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn write_memo_file(output_dir: &str, file_name: &str, content: &str) -> Result<()> {
    fs::create_dir_all(output_dir).context("Failed to create output directory")?;

    let path = Path::new(output_dir).join(file_name);
    let mut file = File::create(&path).context("Failed to create memo file")?;
    writeln!(file, "{}", content).context("Failed to write content to memo file")?;

    Ok(())
}

pub fn sanitize_filename(name: &str) -> Result<String> {
    let re =
        Regex::new(r"[^a-zA-Z0-9]").context("Failed to create regex for filename sanitization")?;
    Ok(re.replace_all(name, "_").into_owned())
}

