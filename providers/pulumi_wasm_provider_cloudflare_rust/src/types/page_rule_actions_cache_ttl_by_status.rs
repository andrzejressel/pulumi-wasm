#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct PageRuleActionsCacheTtlByStatus {
    /// A HTTP code (e.g. `404`) or range of codes (e.g. `400-499`)
    #[builder(into)]
    #[serde(rename = "codes")]
    pub r#codes: Box<String>,
    /// Duration a resource lives in the Cloudflare cache.
    /// - positive number - cache for specified duration in seconds
    #[builder(into)]
    #[serde(rename = "ttl")]
    pub r#ttl: Box<i32>,
}
