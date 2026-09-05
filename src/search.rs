use crate::engines::{duckduckgo::DuckDuckGo, brave::Brave, yahoo::Yahoo, google::Google, SearchEngine};
use crate::models::SearchResultItem;
use futures::stream::{FuturesUnordered, StreamExt};
use reqwest::Client;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tracing::info;
use tokio::time::timeout;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(4);

static RATE_LIMITER: LazyLock<Mutex<HashMap<&'static str, Instant>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new())
});

fn cooldown(engine: &'static str) -> Duration {
    match engine {
        "Google" => Duration::from_secs(3),
        _ => Duration::from_secs(1),
    }
}

async fn throttle(engine: &'static str) {
    loop {
        let wait = {
            let mut map = RATE_LIMITER.lock().unwrap();
            let last = map.entry(engine).or_insert(Instant::now() - cooldown(engine));
            let elapsed = last.elapsed();
            let cool = cooldown(engine);
            if elapsed >= cool {
                *last = Instant::now();
                None
            } else {
                Some(cool - elapsed)
            }
        };
        match wait {
            None => return,
            Some(d) => tokio::time::sleep(d).await,
        }
    }
}

pub async fn perform_search(query: &str) -> Vec<SearchResultItem> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36")
        .timeout(REQUEST_TIMEOUT)
        .build()
        .unwrap_or_else(|_| Client::new());

    let engines: Vec<SearchEngine> = vec![
        SearchEngine::DuckDuckGo(DuckDuckGo),
        SearchEngine::Brave(Brave),
        SearchEngine::Yahoo(Yahoo),
        SearchEngine::Google(Google),
    ];

    let mut results = Vec::new();
    let mut tasks = FuturesUnordered::new();

    for engine in engines {
        let engine_timeout = engine.timeout();
        let q = query.to_string();
        let c = client.clone();
        tasks.push(tokio::spawn(async move {
            let name = engine.name();
            throttle(name).await;
            match timeout(engine_timeout, engine.search(&q, &c)).await {
                Ok(Ok(items)) => {
                    info!("{} returned {} results", name, items.len());
                    items
                }
                Ok(Err(e)) => {
                    eprintln!("Error searching {}: {}", name, e);
                    vec![]
                }
                Err(_) => {
                    eprintln!("Timeout searching {}", name);
                    vec![]
                }
            }
        }));
    }

    while let Some(res) = tasks.next().await {
        if let Ok(mut items) = res {
            results.append(&mut items);
        }
    }

    deduplicate_results(results)
}

fn deduplicate_results(results: Vec<SearchResultItem>) -> Vec<SearchResultItem> {
    let mut seen = HashSet::new();
    let mut deduped = Vec::new();

    for item in results {
        let url = item.url.trim().to_lowercase();
        if seen.contains(&url) {
            if let Some(existing) = deduped.iter_mut().find(|r: &&mut SearchResultItem| r.url.trim().to_lowercase() == url) {
                if !existing.engine.contains(&item.engine) {
                    existing.engine.push_str(", ");
                    existing.engine.push_str(&item.engine);
                }
            }
        } else {
            seen.insert(url);
            deduped.push(item);
        }
    }

    deduped
}
