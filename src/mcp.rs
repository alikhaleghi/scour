use std::io::{self, BufRead, Write};
use serde_json::{json, Value};
use crate::search::perform_search;

#[derive(serde::Deserialize)]
struct JsonRpcRequest { id: Value, method: String, #[serde(default)] params: Option<Value> }

async fn handle_request(req: JsonRpcRequest) -> String {
    match req.method.as_str() {
        "initialize" => serde_json::to_string(&json!({
            "jsonrpc": "2.0", "id": req.id,
            "result": { "protocolVersion": "2024-11-05", "capabilities": { "tools": {} }, "serverInfo": { "name": "scour", "version": env!("CARGO_PKG_VERSION") } }
        })).unwrap(),
        "tools/list" => serde_json::to_string(&json!({
            "jsonrpc": "2.0", "id": req.id,
            "result": { "tools": [{ "name": "web_search", "description": "Search the web", "inputSchema": { "type": "object", "properties": { "query": { "type": "string" } }, "required": ["query"] } }] }
        })).unwrap(),
        "tools/call" => {
            let query = req.params.and_then(|p| p.get("arguments").and_then(|a| a.get("query")).and_then(|q| q.as_str())).unwrap_or("");
            let results = perform_search(query).await;
            serde_json::to_string(&json!({
                "jsonrpc": "2.0", "id": req.id,
                "result": { "content": [{ "type": "text", "text": serde_json::to_string(&results).unwrap() }] }
            })).unwrap()
        }
        _ => serde_json::to_string(&json!({"jsonrpc": "2.0", "id": req.id, "error": { "code": -32601, "message": "Method not found" }})).unwrap(),
    }
}

pub async fn run() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.trim().is_empty() { continue; }
            if let Ok(req) = serde_json::from_str::<JsonRpcRequest>(&line) {
                println!("{}", handle_request(req).await);
                io::stdout().flush().unwrap();
            }
        }
    }
}
