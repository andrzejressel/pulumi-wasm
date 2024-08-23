#[derive(serde::Serialize)]
pub struct RateLimitMatch {
    #[serde(rename = "request")]
    pub r#request: Box<Option<crate::types::RateLimitMatchRequest>>,
    #[serde(rename = "response")]
    pub r#response: Box<Option<crate::types::RateLimitMatchResponse>>,
}
