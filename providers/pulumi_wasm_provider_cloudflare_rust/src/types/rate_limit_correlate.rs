#[derive(serde::Serialize)]
pub struct RateLimitCorrelate {
    #[serde(rename = "by")]
    pub r#by: Box<Option<String>>,
}
