# Trash Cleaner

A powerful command-line tool written in Rust for cleaning up build artifacts and dependency directories (like `node_modules`, `target`, `__pycache__`) that haven't been modified in a specified number of days.

## Features

- 🚀 Multi-threaded scanning for fast directory discovery
- 📦 Support for multiple project types (Node.js, Rust/Cargo, Python)
- 🗑️ Moves files to system trash instead of permanent deletion
- 🎯 Interactive selection with "Select All" option
- 📊 Shows directory sizes before deletion
- 🔍 Dry-run mode for previewing what would be deleted

## Installation

### From Source

```bash
git clone <your-repo-url>
cd hello-rust
cargo install --path .
```

### Usage

```bash
# Scan for node_modules directories older than 60 days (default)
trash-cleaner --path /path/to/scan --type node

# Scan for Rust target directories older than 30 days
trash-cleaner --path /path/to/scan --type cargo --days 30

# Dry run to preview what would be deleted
trash-cleaner --path /path/to/scan --type node --dry-run

# Scan Python __pycache__ directories
trash-cleaner --path /path/to/scan --type python --days 7
```

## Options

- `--path, -p`: Path to scan (default: current directory)
- `--days, -d`: Number of days since last modification (default: 60)
- `--type, -t`: Project type: `node`, `cargo`, or `python` (default: `node`)
- `--dry-run`: Preview what would be deleted without actually deleting

## Supported Project Types

- **Node.js/NPM**: Scans for `node_modules` directories
- **Rust/Cargo**: Scans for `target` directories
- **Python**: Scans for `__pycache__` directories

## How It Works

1. Scans the specified directory recursively for target directories
2. Checks the last modification time of each directory
3. Filters directories older than the specified number of days
4. Shows an interactive menu to select directories to clean
5. Moves selected directories to the system trash (not permanently deleted)

## Safety

- Files are moved to your system trash, not permanently deleted
- You can recover them from the trash if needed
- Use `--dry-run` to preview before deleting

## License

MIT
