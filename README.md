# Memos CLI

Memos CLI is a command-line interface tool for managing memos through a self-hosted API. It allows users to retrieve, create, and update memos efficiently.

## Features

- Retrieve all memos from the API
- Create new memos (placeholder functionality)
- Update existing memos (single or bulk update)
- Save memos as local files
- Maintain memo metadata in a JSON file

## Installation

1. Ensure you have Rust and Cargo installed on your system.
2. Clone this repository:
   ```
   git clone https://github.com/yourusername/memos-cli.git
   cd memos-cli
   ```
3. Build the project:
   ```
   cargo build --release
   ```
4. The binary will be available in `target/release/memos-cli`

## Usage

Before using the CLI, make sure to set the following environment variables:
- `MEMOAPI`: Your API authentication token
- `MEMO_DOMAIN`: The domain of your self-hosted memo instance

You can set these in a `.env` file in the project root.

### General Command Structure

```
memos-cli [OPTIONS] --endpoint <ENDPOINT>
```

### Options

- `-m, --memo-domain <MEMO_DOMAIN>`: Memo domain of self-hosted instance
- `-e, --endpoint <ENDPOINT>`: API endpoint to call (getMemos, createMemo, updateMemo)
- `-o, --output-dir <OUTPUT_DIR>`: Output directory for files [default: content]
- `-v, --verbose`: Verbose mode
- `-n, --memo-name <MEMO_NAME>`: Memo name or uid (for single memo operations)
- `-c, --content <CONTENT>`: Optional content for memo creation or update
- `--bulk-update`: Flag for bulk update of memos

### Examples

1. Retrieve all memos:
   ```
   memos-cli --endpoint getMemos
   ```

2. Update a single memo:
   ```
   memos-cli --endpoint updateMemo --memo-name <MEMO_UID> --content "Updated content"
   ```

3. Bulk update all memos:
   ```
   memos-cli --endpoint updateMemo --bulk-update
   ```

## Project Structure

- `src/`
  - `main.rs`: Entry point of the application
  - `cli.rs`: CLI argument parsing
  - `utils.rs`: Utility functions
  - `api/`
    - `mod.rs`: API module definitions
    - `getMemos.rs`: Functions for retrieving memos
    - `createMemo.rs`: Functions for creating memos (placeholder)
    - `updateMemo.rs`: Functions for updating memos

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
