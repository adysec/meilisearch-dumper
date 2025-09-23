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
# Import single index (JSONL)
./meilisearch-dumper --index movies --files data/movies.jsonl

# Import multiple indexes
./meilisearch-dumper \
  --index movies --files data/movies.jsonl \
  --index books --files data/books.jsonl

### Merge Multiple Files into One Index

支持两种合并方式：

1. 逗号分隔多个文件（同一次 `--index`）：

```bash
./meilisearch-dumper \
  --index movies --files data/movies_part1.jsonl,data/movies_part2.jsonl
```

2. 同名 `--index` 多次出现（会合并文件，属性以最后一次为准）：

```bash
./meilisearch-dumper \
  --index movies --files data/movies_part1.jsonl \
  --index movies --files data/movies_part2.jsonl \
    --displayed title,year \
    --searchable title \
    --filterable year
```

注意：两种方式可以混用，最终会将所有文件顺序合并到同一个 `documents.jsonl` 中。

### Attribute Override Rule

当同名 `--index` 多次出现时：
- 文件会合并（先出现的文件先写入，后出现的文件追加）。
- `--displayed` / `--searchable` / `--filterable` 以最后一次出现为准。

### Tips: Why Search May Return 0 Results

若未设置 `--searchable`，将写入空数组，等价于禁用文本搜索，导致 `/search` 无命中。请传入可检索字段，例如：

```bash
./meilisearch-dumper \
  --index movies --files data/movies.jsonl \
  --searchable title,genre
```

### Troubleshooting

- Cannot connect to server after import:
  - 确认 MeiliSearch 正在运行且监听端口（默认 7700）。
  - 使用 `--http-addr` 指定端口并在客户端请求中对应调整。
- Database already exists 错误：
  - MeiliSearch 启动时指定空目录：`--db-path /tmp/ms-test`。
  - 或删除/备份既有数据目录后再导入。
 - 搜索无结果：
  - 确认传入了 `--searchable` 字段，并重新导入生成的 dump。
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
| `--files` | Input JSONL file path | ✅ | `data/movies.jsonl` |
| `--displayed` | Display attributes (comma-separated) | ❌ | `title,year,genre` |
| `--searchable` | Searchable attributes (comma-separated) | ❌ | `title,genre` |
| `--filterable` | Filterable attributes (comma-separated) | ❌ | `year,genre` |

### Input File Format (JSONL)

输入文件必须为 JSON Lines（JSONL）格式：每行一个 JSON 对象，例如：

```json
{"id": 1, "title": "The Shawshank Redemption", "year": 1994, "genre": "Drama"}
{"id": 2, "title": "The Godfather", "year": 1972, "genre": "Crime"}
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

### v0.2.0
- 更换输入为 JSONL（每行一个对象）
- 支持 `--files` 逗号分隔多文件合并
- 支持同名 `--index` 多次出现进行合并，属性以最后一次为准
- 文档补充使用建议与故障排查

### v0.1.0
- Initial release
- Support for multi-index dump generation
- Support for custom index attribute configuration
- Generate dump files conforming to MeiliSearch V6 format
