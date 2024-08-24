#[derive(serde::Serialize)]
pub struct RateLimitMatchResponse {
    /// List of HTTP headers maps to match the origin response on.
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<std::collections::HashMap<String, String>>>>,
    /// Only count traffic that has come from your origin servers. If true, cached items that Cloudflare serve will not count towards rate limiting.
    #[serde(rename = "originTraffic")]
    pub r#origin_traffic: Box<Option<bool>>,
    /// HTTP Status codes, can be one, many or indicate all by not providing this value.
    #[serde(rename = "statuses")]
    pub r#statuses: Box<Option<Vec<i32>>>,
}
