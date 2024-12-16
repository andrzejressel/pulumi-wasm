#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RateLimitMatchResponse {
    /// List of HTTP headers maps to match the origin response on.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<std::collections::HashMap<String, String>>>>,
    /// Only count traffic that has come from your origin servers. If true, cached items that Cloudflare serve will not count towards rate limiting.
    #[builder(into, default)]
    #[serde(rename = "originTraffic")]
    pub r#origin_traffic: Box<Option<bool>>,
    /// HTTP Status codes, can be one, many or indicate all by not providing this value.
    #[builder(into, default)]
    #[serde(rename = "statuses")]
    pub r#statuses: Box<Option<Vec<i32>>>,
}
