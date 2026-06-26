# Scour

A lightning-fast search API in Rust that scrapes Brave and Yahoo (Bing) concurrently and returns normalized JSON. Runs as an HTTP server or MCP server for AI agents. No API keys required.

## How It Works

```
HTTP/MCP request → concurrent scrapers → Brave + Yahoo → merged results as JSON
```

Each engine has a 5-second timeout so a single hung engine doesn't block results.

## Quick Start

```bash
cargo build --release
./target/release/scour
curl "http://localhost:10080/search?q=hello+world"
```

## HTTP API

```
GET /search?q=<query>
```

Response:
```json
{
  "query": "rust tokio",
  "results": [
    {
      "title": "Tokio - An asynchronous Rust runtime",
      "url": "https://tokio.rs/",
      "snippet": "Tokio is a library for writing fast...",
      "engine": "Yahoo"
    }
  ]
}
```

## MCP Server Mode

For AI agents (Claude Desktop, Gemini CLI, Cursor):

```bash
./scour --mcp
```

Configure:
```json
{
  "mcpServers": {
    "scour": {
      "command": "/path/to/scour",
      "args": ["--mcp"]
    }
  }
}
```

## Engines

| Engine | Status |
|--------|--------|
| Brave | ✅ Working |
| Yahoo (Bing) | ✅ Working |

## Build

```bash
cargo build --release
```

Requires: Rust toolchain.

## License

CC BY-NC 4.0
