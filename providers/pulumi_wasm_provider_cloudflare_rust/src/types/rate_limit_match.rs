#[derive(serde::Serialize)]
pub struct RateLimitMatch {
    /// Matches HTTP requests (from the client to Cloudflare).
    #[serde(rename = "request")]
    pub r#request: Box<Option<crate::types::RateLimitMatchRequest>>,
    /// Matches HTTP responses before they are returned to the client from Cloudflare. If this is defined, then the entire counting of traffic occurs at this stage.
    #[serde(rename = "response")]
    pub r#response: Box<Option<crate::types::RateLimitMatchResponse>>,
}
