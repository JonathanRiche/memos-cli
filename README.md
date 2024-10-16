# Memo CLI

Memo CLI is a command-line interface tool for managing memos using the Memo API. It allows users to retrieve, create, and update memos efficiently.

## Features

- Retrieve all memos and save them locally
- Create new memos
- Update existing memos (single or bulk update)
- Verbose mode for detailed output
- Secure API key management using environment variables

## Installation

1. Ensure you have Rust and Cargo installed on your system.
2. Clone this repository:
   ```
   git clone https://github.com/yourusername/memo-cli.git
   cd memo-cli
   ```
3. Build the project:
   ```
   cargo build --release
   ```

## Configuration

1. Create a `.env` file in the project root directory.
2. Add your Memo API key to the `.env` file:
   ```
   MEMOAPI=your_api_key_here
   ```

## Usage

Run the CLI tool using the following command structure:

```
cargo run -- [OPTIONS] --endpoint <ENDPOINT>
```

### Options

- `-e, --endpoint <ENDPOINT>`: Specify the API endpoint to call (required)
  - Available endpoints: `getMemos`, `createMemo`, `updateMemo`
- `-o, --output-dir <OUTPUT_DIR>`: Set the output directory for files (default: "content")
- `-v, --verbose`: Enable verbose mode
- `-m, --memo-name <MEMO_NAME>`: Specify the memo name or UID (required for single memo update)
- `-c, --content <CONTENT>`: Provide content for memo creation or update
- `--bulk-update`: Enable bulk update of memos

### Examples

1. Retrieve all memos:
   ```
   cargo run -- --endpoint getMemos
   ```

2. Create a new memo:
   ```
   cargo run -- --endpoint createMemo --content "New memo content"
   ```

3. Update a single memo:
   ```
   cargo run -- --endpoint updateMemo --memo-name memo_123 --content "Updated content"
   ```

4. Bulk update all memos:
   ```
   cargo run -- --endpoint updateMemo --bulk-update
   ```

5. Use verbose mode:
   ```
   cargo run -- --endpoint getMemos --verbose
   ```

## Project Structure

- `src/main.rs`: Entry point of the application
- `src/cli.rs`: CLI argument parsing
- `src/utils.rs`: Utility functions
- `src/api/`: API-related modules
  - `mod.rs`: API module definitions
  - `getMemos.rs`: Retrieve memos functionality
  - `createMemo.rs`: Create memo functionality
  - `updateMemo.rs`: Update memo functionality

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

