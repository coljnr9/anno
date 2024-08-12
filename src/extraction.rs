// Possible extraction services:
// - pipfeed
// - extractorapi

use anyhow::Result;
use reqwest;
use serde::{self, Deserialize, Serialize};
use tracing::info;
use url::Url;

pub struct ArticleRecord {
    pub title: String,
    pub html: String,
    pub text: String,
}
// Where do I want this to run? I suppose the client can just make the requests for now...
pub trait Extract {
    async fn extract_url(&self, url: &Url) -> Result<ArticleRecord>;
}

pub struct ExtractorApi {
    pub api_key: String,
    pub endpoint: Url,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtractorApiResponse {
    pub url: String,
    pub status: String,
    #[serde(rename = "status_code")]
    pub status_code: i64,
    pub domain: String,
    pub title: String,
    pub author: Vec<String>,
    #[serde(rename = "date_published")]
    pub date_published: Option<String>,
    pub images: Vec<String>,
    pub videos: Vec<String>,
    pub text: String,
    pub html: String,
}

impl Extract for ExtractorApi {
    async fn extract_url(&self, url: &Url) -> Result<ArticleRecord> {
        info!("Running article extraction for {url}");

        let mut api_url = self.endpoint.clone();

        api_url
            .query_pairs_mut()
            .append_pair("apikey", &self.api_key);
        api_url.query_pairs_mut().append_pair("url", &url.as_str());

        let raw_res = reqwest::get(api_url).await?;

        let res: ExtractorApiResponse = match raw_res.json().await {
            Ok(a) => a,
            Err(e) => {
                dbg!(e);
                panic!()
            }
        };

        let record = ArticleRecord {
            title: res.title,
            html: res.html,
            text: res.text,
        };
        Ok(record)
    }
}
