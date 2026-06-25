use crate::engines::{duckduckgo::DuckDuckGo, brave::Brave, yahoo::Yahoo, SearchEngine};
use crate::models::SearchResultItem;
use futures::stream::{FuturesUnordered, StreamExt};
use rquest::Client;
use std::time::Duration;
use tracing::info;
use tokio::time::timeout;

const ENGINE_TIMEOUT: Duration = Duration::from_secs(10);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(8);

pub async fn perform_search(query: &str) -> Vec<SearchResultItem> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .timeout(REQUEST_TIMEOUT)
        .build()
        .unwrap_or_else(|_| Client::new());
    let engines: Vec<SearchEngine> = vec![
        SearchEngine::DuckDuckGo(DuckDuckGo),
        SearchEngine::Brave(Brave),
        SearchEngine::Yahoo(Yahoo),
    ];
    let mut results = Vec::new();
    let mut tasks = FuturesUnordered::new();
    for engine in engines {
        let q = query.to_string();
        let c = client.clone();
        tasks.push(tokio::spawn(async move {
            let name = engine.name();
            match timeout(ENGINE_TIMEOUT, engine.search(&q, &c)).await {
                Ok(Ok(items)) => { info!("{} returned {} results", name, items.len()); items }
                Ok(Err(e)) => { eprintln!("Error searching {}: {}", name, e); vec![] }
                Err(_) => { eprintln!("Timeout searching {}", name); vec![] }
            }
        }));
    }
    while let Some(res) = tasks.next().await {
        if let Ok(mut items) = res { results.append(&mut items); }
    }
    results
}
