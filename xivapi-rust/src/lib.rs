use ::serde::{Deserialize, Serialize};
use reqwest::Client;

pub struct APIClient {
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct Pagination {
    #[serde(rename = "Page")]
    page: i32,
    #[serde(rename = "PageNext")]
    page_next: Option<i32>,
    #[serde(rename = "PagePrev")]
    page_prev: Option<i32>,
    #[serde(rename = "PageTotal")]
    page_total: i32,
    #[serde(rename = "Results")]
    results: i32,
    #[serde(rename = "ResultsPerPage")]
    results_per_page: i32,
    #[serde(rename = "ResultsTotal")]
    results_total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchItem {
    #[serde(rename = "ID")]
    id: i32,
    #[serde(rename = "Icon")]
    icon: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Url")]
    url: String,
    #[serde(rename = "UrlType")]
    url_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "Pagination")]
    pagination: Pagination,
    #[serde(rename = "Results")]
    results: Vec<SearchItem>,
    #[serde(rename = "SpeedMs")]
    speed_ms: i32,
}

impl APIClient {
    pub fn new() -> Self {
        let client = Client::new();
        APIClient { client }
    }

    pub async fn string_search(
        &self,
        indexes: Option<Vec<&str>>,
        string: &str,
        string_algorithm: Option<&str>,
        page: i32,
    ) -> Result<SearchResponse, reqwest::Error> {
        let mut search = Vec::new();

        if let Some(indexes) = &indexes {
            search.push(format!("indexes={}", indexes.join(",")));
        }

        search.push(format!("string={}", string));

        if let Some(algorithm) = string_algorithm {
            search.push(format!("string_algo={}", algorithm));
        }

        search.push(format!("page={}", page));

        let response: SearchResponse = self
            .client
            .get(format!("https://xivapi.com/search?{}", search.join("&")))
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}

impl Default for APIClient {
    fn default() -> Self {
        Self::new()
    }
}
