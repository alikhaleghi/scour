# Scour

A lightning-fast search API in Rust that scrapes Brave and Yahoo (Bing) concurrently and returns normalized JSON. Runs as an HTTP server or MCP server for AI agents. No API keys required.

## How It Works

```
HTTP/MCP request → concurrent scrapers → Brave + Yahoo + DuckDuckGo* → deduped results as JSON
```

Each engine runs with an independent timeout (DuckDuckGo: 2s, Brave/Yahoo: 5s) so a single hung engine doesn't block results. Duplicate URLs across engines are merged into one result listing all contributing engines.

> DuckDuckGo is network-blocked from the production server (Azure IP `20.204.244.192` hangs on TCP connect). It always times out but never blocks other engines.

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
      "engine": "Brave"
    },
    {
      "title": "Tokio - The Tokio Guide",
      "url": "https://tokio.rs/docs",
      "snippet": "A practical guide...",
      "engine": "Brave, Yahoo"
    }
  ]
}
```

The `engine` field may contain comma-separated names when the same URL is returned by multiple engines.

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

Exposes one tool `web_search` that takes a `query` string and returns aggregated results.

## Engines

| Engine | Status |
|--------|--------|
| Brave | ✅ Working |
| Yahoo (Bing) | ✅ Working |
| DuckDuckGo | ❌ Network blocked (production) — always times out |

## Performance

Tested against live API (`api.ahur.ir/search`):

| Metric | Result |
|--------|--------|
| Avg latency (5 samples) | ~1.2s |
| Max concurrent users | 10 at 100% success |
| Batch throughput | 30 req / ~4s |

## Build

```bash
cargo build --release
```

Requires: Rust toolchain.

## License

CC BY-NC 4.0
