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

## MCP Server Mode
```bash
./scour --mcp
```
