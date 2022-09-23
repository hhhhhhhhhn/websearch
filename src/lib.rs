use reqwest;
use reqwest::Url;
use scraper::{Html, Selector};

pub async fn searx(query: &str) -> Option<Vec<String>> {
    let url = Url::parse_with_params(
        "https://search.ononoki.org/search",
        &[("q", query)]
    ).ok()?;
    let request = reqwest::get(url).await.ok()?;
    let document = Html::parse_document(&request.text().await.ok()?);
    let link_selector = Selector::parse("article.result a.url_wrapper").unwrap();

    return Some(
        document.select(&link_selector)
            .filter_map(|e| e.value().attr("href").map(|x| x.to_string()))
            .collect()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_searx() {
        let results = searx("wikipedia home page").await.unwrap();
        assert!(results.len() > 0);
        assert!(results.iter().any(|link| link.contains("wikipedia.org")));
    }
}
