use reqwest::Client;
use scraper::{Html, Selector};
use crate::models::SearchResultItem;

pub struct Brave;

impl Brave {
    pub fn name(&self) -> &'static str {
        "Brave"
    }

    pub async fn search(&self, query: &str, client: &Client) -> Result<Vec<SearchResultItem>, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("https://search.brave.com/search?q={}", urlencoding::encode(query));

        let response = client.get(&url).send().await?.text().await?;

        let document = Html::parse_document(&response);

        let result_selector = Selector::parse(".snippet[data-type=\"web\"]").unwrap();
        let title_selector = Selector::parse(".search-snippet-title").unwrap();
        let link_selector = Selector::parse("a[href^=\"https://\"]").unwrap();
        let snippet_selector = Selector::parse(".generic-snippet").unwrap();

        let mut results = Vec::new();

        for element in document.select(&result_selector) {
            let title = element
                .select(&title_selector)
                .next()
                .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
                .unwrap_or_default();

            let url = element
                .select(&link_selector)
                .next()
                .and_then(|el| el.value().attr("href"))
                .unwrap_or("")
                .to_string();

            let snippet = element
                .select(&snippet_selector)
                .next()
                .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
                .unwrap_or_default();

            if !url.is_empty() && !title.is_empty() {
                results.push(SearchResultItem {
                    title,
                    url,
                    snippet,
                    engine: self.name().to_string(),
                });
            }
        }

        Ok(results)
    }
}
