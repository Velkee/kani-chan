use reqwest::{Client, Response};

pub struct APIClient {
    client: Client,
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
    ) -> Result<Response, reqwest::Error> {
        let mut search: Vec<String> = vec![];

        let index_string = match indexes {
            Some(indexes) => format!("indexes={}", indexes.join(", ")),
            None => String::from(""),
        };

        if !index_string.is_empty() {
            search.push(index_string);
        }

        search.push(format!("string={}", string));

        let string_algo = match string_algorithm {
            Some(algorighm) => format!("string_algo={algorighm}"),
            None => String::from(""),
        };

        if !string_algo.is_empty() {
            search.push(string_algo)
        }

        let page_string = format!("page={page}");

        search.push(page_string);

        let search_string = search.join("&");

        self.client
            .get(format!("https://xivapi.com/search?{search_string}"))
            .send()
            .await
    }
}

impl Default for APIClient {
    fn default() -> Self {
        Self::new()
    }
}
