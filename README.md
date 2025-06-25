# MeiliSearch Dumper

A high-performance Rust tool for generating MeiliSearch dump files from JSON data.

## Features

- 🚀 **High Performance**: Built with Rust for excellent memory efficiency and processing speed
- 📦 **Multi-Index Support**: Process multiple indexes and files simultaneously
- ⚙️ **Flexible Configuration**: Independent configuration of display, search, and filter attributes for each index
- 🔧 **Zero Dependencies**: Single binary file after compilation, no runtime dependencies
- 📋 **Complete Structure**: Generates complete dump files conforming to MeiliSearch V6 format

## Installation

### Build from Source

```bash
# Clone repository
git clone https://github.com/adysec/meilisearch-dumper.git
cd meilisearch-dumper

# Build release version
cargo build --release

# Binary file located at target/release/meilisearch-dumper
```

### System Requirements

- Rust 1.70+
- Cargo

## Usage

### Basic Usage

```bash
# Import single index
./meilisearch-dumper --index movies --files data/movies.json

# Import multiple indexes
./meilisearch-dumper \
  --index movies --files data/movies.json \
  --index books --files data/books.json
```

### Advanced Usage (with Attribute Configuration)

```bash
# Configure display, search, and filter attributes for each index
./meilisearch-dumper \
  --index movies --files data/movies.json \
    --displayed title,year,genre \
    --searchable title,genre \
    --filterable year,genre \
  --index books --files data/books.json \
    --displayed title,author,year \
    --searchable title,author \
    --filterable year
```

### Parameter Description

| Parameter | Description | Required | Example |
|-----------|-------------|----------|---------|
| `--index` | Index name | ✅ | `movies` |
| `--files` | Input JSON file path | ✅ | `data/movies.json` |
| `--displayed` | Display attributes (comma-separated) | ❌ | `title,year,genre` |
| `--searchable` | Searchable attributes (comma-separated) | ❌ | `title,genre` |
| `--filterable` | Filterable attributes (comma-separated) | ❌ | `year,genre` |

### Input File Format

Input files must be JSON files containing an array of objects:

```json
[
  {
    "id": 1,
    "title": "The Shawshank Redemption",
    "year": 1994,
    "genre": "Drama"
  },
  {
    "id": 2,
    "title": "The Godfather",
    "year": 1972,
    "genre": "Crime"
  }
]
```

## Output

The tool generates the following files:

- `meilisearch_dump/` - Dump directory structure
- `meilisearch_dump.dump` - Compressed dump file (can be directly imported to MeiliSearch)

### Dump File Structure

```
meilisearch_dump/
├── metadata.json              # Metadata information
├── instance_uid.uuid          # Instance ID
├── keys.jsonl                 # API keys
├── network.json               # Network configuration
├── experimental-features.json # Experimental features
├── batches/
│   └── queue.jsonl           # Batch processing queue
├── tasks/
│   └── queue.jsonl           # Task queue
└── indexes/
    ├── movies/
    │   ├── metadata.json     # Index metadata
    │   ├── settings.json     # Index settings
    │   └── documents.jsonl   # Document data
    └── books/
        ├── metadata.json
        ├── settings.json
        └── documents.jsonl
```

## Import to MeiliSearch

```bash
# Start MeiliSearch and import dump
./meilisearch --import-dump meilisearch_dump.dump

# Verify import results
curl -X GET "http://localhost:7700/indexes" \
  -H "Authorization: Bearer 0ad8bdfc-36fc-4005-810d-c7abd3a8a2e2"

# Search test
curl -X GET "http://localhost:7700/indexes/movies/search" \
  -H "Authorization: Bearer 0ad8bdfc-36fc-4005-810d-c7abd3a8a2e2" \
  -H "Content-Type: application/json" \
  -d '{"q": "action"}'
```

## Performance Characteristics

- **Memory Efficiency**: Stream processing of large files to avoid memory overflow
- **Concurrent Processing**: Support for parallel processing of multiple indexes
- **Fast Packaging**: Efficient tar.gz compression
- **Type Safety**: Rust compile-time checking to reduce runtime errors

## Comparison with API Import Performance and Speed

| Feature | Dump Import | API Import |
|---------|-------------|------------|
| Import Speed | ⚡ Very Fast (seconds) | 🐌 Slow (minutes) |
| Network Dependency | 🚫 No network requests | 📡 Requires HTTP requests |
| Error Handling | ✅ Atomic operations | ❌ Partial failure risk |
| Large Data Volume | 💪 Supports GB-level data | 📉 Limited by network |
| Offline Operation | ✅ Completely offline | ❌ Requires online |
| Batch Processing | 📦 One-time import | 🔄 Batch processing |
| Index Settings | ⚙️ Complete retention | 🔧 Requires reconfiguration |

## Comparison with meilisearch-importer Performance and Speed

> **Note**: [meilisearch-importer](https://github.com/meilisearch/meilisearch-importer) is the official MeiliSearch Rust CLI tool for direct API data import. While `meilisearch-dumper` focuses on generating offline dump files, they have different but complementary purposes.

| Feature | meilisearch-dumper | meilisearch-importer |
|---------|-------------------|---------------------|
| Function Purpose | 🎯 Generate dump files | 📤 Direct API import |
| Import Method | 📦 Offline dump import | 🌐 Online API import |
| Network Dependency | 🚫 No network requests | 📡 Requires HTTP connection |
| Large Data Volume | 💪 Supports GB-level data | 📉 Limited by network |
| Error Handling | ✅ Atomic operations | ❌ Partial failure risk |
| Index Settings | ⚙️ Complete configuration retention | 🔧 Requires reconfiguration |
| Offline Operation | ✅ Completely offline | ❌ Requires online |
| Deployment Scenario | 🏢 Production environment | 🧪 Development testing |
| Batch Processing | 📦 One-time import | 🔄 Batch upload |
| Progress Display | 📊 Simple progress | 📈 Detailed progress bar |
| Retry Mechanism | 🚫 No retry needed | 🔄 Automatic retry |
| Use Case | 🎯 Large-scale data migration | 🚀 Quick data import |

## Development

### Project Structure

```
src/
├── main.rs      # Main program entry
├── args.rs      # Command line argument parsing
└── dump.rs      # Dump file generation logic
```

### Build

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Code linting
cargo clippy
```

## Contributing

Issues and Pull Requests are welcome!

## Changelog

### v0.1.0
- Initial release
- Support for multi-index dump generation
- Support for custom index attribute configuration
- Generate dump files conforming to MeiliSearch V6 format
