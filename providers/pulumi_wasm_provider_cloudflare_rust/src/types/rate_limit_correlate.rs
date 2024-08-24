#[derive(serde::Serialize)]
pub struct RateLimitCorrelate {
    /// If set to 'nat', NAT support will be enabled for rate limiting. Available values: `nat`.
    #[serde(rename = "by")]
    pub r#by: Box<Option<String>>,
}
