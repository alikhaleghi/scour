# Quick Start

## Build and Run

```bash
cargo build --release
./target/release/scour
```

## Usage

```bash
curl -s "http://localhost:10080/search?q=rust+lang" | python3 -m json.tool
```

### Example Output

```json
{
  "query": "rust lang",
  "results": [
    {
      "title": "Rust Programming Language",
      "url": "https://rust-lang.org/",
      "snippet": "Rust is a fast, reliable, and productive programming language...",
      "engine": "Brave"
    }
  ]
}
```

## Using with AI Agents

### Run as MCP Server

```bash
./scour --mcp
```

### Claude Desktop

Add to `claude_desktop_config.json`:
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

### Test MCP Handshake

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | ./scour --mcp
```

Expected response:
```json
{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2024-11-05","capabilities":{"tools":{}},"serverInfo":{"name":"scour","version":"0.1.0"}}}
```

## Docker

```bash
docker build -t scour:latest .
docker run -d -p 10080:10080 --name scour-api scour:latest
```

## Environment

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `10080` | HTTP server port |
