#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct RateLimitMatch {
    /// Matches HTTP requests (from the client to Cloudflare).
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "request")]
    pub r#request: Box<Option<crate::types::RateLimitMatchRequest>>,
    /// Matches HTTP responses before they are returned to the client from Cloudflare. If this is defined, then the entire counting of traffic occurs at this stage.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "response")]
    pub r#response: Box<Option<crate::types::RateLimitMatchResponse>>,
}
