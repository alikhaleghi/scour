use chromiumoxide::browser::Browser;
use futures::StreamExt;
use crate::models::SearchResultItem;

const CDP_URL: &str = "http://localhost:9222";

pub struct Google;

impl Google {
    pub fn name(&self) -> &'static str {
        "Google"
    }

    pub async fn search(&self, query: &str, _client: &reqwest::Client) -> Result<Vec<SearchResultItem>, Box<dyn std::error::Error + Send + Sync>> {
        let (browser, mut handler) = Browser::connect(CDP_URL).await?;

        tokio::spawn(async move {
            while let Some(_) = handler.next().await {}
        });

        let page = browser.new_page("about:blank").await?;
        let url = format!("https://www.google.com/search?q={}&hl=en", urlencoding::encode(query));
        page.goto(&url).await?;
        page.wait_for_navigation().await?;

        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        let title: String = page.evaluate("document.title").await?.into_value().unwrap_or_default();
        let current_url: String = page.evaluate("window.location.href").await?.into_value().unwrap_or_default();
        eprintln!("[Google] Page title: {:?}, URL: {:?}", title, current_url);

        let results: Vec<RawResult> = page.evaluate(EXTRACT_JS).await?.into_value().unwrap_or_default();
        eprintln!("[Google] Extracted {} results via JS", results.len());

        page.close().await?;

        let search_results: Vec<SearchResultItem> = results
            .into_iter()
            .filter(|r| !r.title.is_empty() && !r.url.is_empty())
            .map(|r| SearchResultItem {
                title: r.title,
                url: r.url,
                snippet: r.snippet,
                engine: self.name().to_string(),
            })
            .collect();

        Ok(search_results)
    }
}

#[derive(serde::Deserialize)]
struct RawResult {
    title: String,
    url: String,
    snippet: String,
}

const EXTRACT_JS: &str = r#"
(() => {
    const results = [];
    const items = document.querySelectorAll('div.g, div[data-hveid], div[data-sokoban-container]');
    for (const el of items) {
        const h3 = el.querySelector('h3');
        if (!h3) continue;
        const link = h3.closest('a') || el.querySelector('a[href^="https://"]');
        const url = link ? (link.getAttribute('href') || '') : '';
        const snippetEl = el.querySelector('div[data-sncf], span.aCOpRe, div.VwiC3b, div[style*="line-clamp"]');
        const snippet = snippetEl ? snippetEl.textContent.trim() : '';
        results.push({
            title: h3.textContent.trim(),
            url: url,
            snippet: snippet
        });
    }
    return results;
})()
"#;
