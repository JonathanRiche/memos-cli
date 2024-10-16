use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Memo domain of self-hosted instance
    #[arg(short, long)]
    pub memo_domain: Option<String>,
    /// API endpoint to call
    /// getMemos - Get all memos
    /// createMemo - Create a new memo
    /// updateMemo - Update a memo
    #[arg(short, long)]
    pub endpoint: String,

    /// Output directory for files
    #[arg(short, long, default_value = "content")]
    pub output_dir: String,

    /// Verbose mode
    #[arg(short, long)]
    pub verbose: bool,

    /// Memo name or uid
    #[arg(short, long)]
    pub memo_name: Option<String>,

    // Optional content if not pulling from existing files
    #[arg(short, long)]
    pub content: Option<String>,

    //Bulk update memos flag
    #[arg(long)]
    pub bulk_update: bool,
}
