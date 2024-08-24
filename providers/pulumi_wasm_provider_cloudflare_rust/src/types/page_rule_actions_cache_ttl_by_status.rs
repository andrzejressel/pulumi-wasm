#[derive(serde::Serialize)]
pub struct PageRuleActionsCacheTtlByStatus {
    /// A HTTP code (e.g. `404`) or range of codes (e.g. `400-499`)
    #[serde(rename = "codes")]
    pub r#codes: Box<String>,
    /// Duration a resource lives in the Cloudflare cache.
    /// - positive number - cache for specified duration in seconds
    #[serde(rename = "ttl")]
    pub r#ttl: Box<i32>,
}
