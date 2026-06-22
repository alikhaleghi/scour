# Scour

A lightning-fast search API in Rust that scrapes DuckDuckGo, Yahoo, and Brave concurrently and returns normalized JSON. No API keys required. Runs as an HTTP server or MCP server for AI agents.

## Quick Start
```bash
cargo build --release
./target/release/scour
curl "http://localhost:10080/search?q=hello+world"
```

## Supported Engines
- DuckDuckGo
- Yahoo (Bing)
- Brave Search

## License
CC BY-NC 4.0
