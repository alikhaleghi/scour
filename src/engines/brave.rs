use reqwest::Client;
use scraper::{Html, Selector};
use crate::models::SearchResultItem;

pub struct Brave;

fn extract_text(element: &scraper::ElementRef, selector: &Selector) -> String {
    element
        .select(selector)
        .next()
        .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
        .unwrap_or_default()
}

impl Brave {
    pub fn name(&self) -> &'static str {
        "Brave"
    }

    pub async fn search(&self, query: &str, client: &Client) -> Result<Vec<SearchResultItem>, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("https://search.brave.com/search?q={}", urlencoding::encode(query));

        let response = client.get(&url).send().await?.text().await?;

        let document = Html::parse_document(&response);

        let result_selector = Selector::parse(".snippet, [data-result], .result").unwrap();
        let title_selector = Selector::parse(".title, .snippet-title, h2, [class*=\"title\"]").unwrap();
        let link_selector = Selector::parse("a[href^=\"https://\"], a[href^=\"http://\"]").unwrap();
        let snippet_selector = Selector::parse(
            ".snippet-description, .snippet-content, .description, \
             .snippet-description p, .content, .result-snippet, \
             p.snippet, [class*=\"description\"], [class*=\"snippet\"]"
        ).unwrap();

        let mut results = Vec::new();

        for element in document.select(&result_selector) {
            let title = extract_text(&element, &title_selector);
            let url = element
                .select(&link_selector)
                .next()
                .and_then(|el| el.value().attr("href"))
                .unwrap_or("")
                .to_string();

            let mut snippet = extract_text(&element, &snippet_selector);

            if snippet.is_empty() {
                let all_text: String = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
                if !title.is_empty() {
                    snippet = all_text
                        .replace(&title, "")
                        .trim()
                        .splitn(2, |c: char| c.is_whitespace() && c == ' ')
                        .nth(1)
                        .unwrap_or("")
                        .trim()
                        .to_string();
                }
            }

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
