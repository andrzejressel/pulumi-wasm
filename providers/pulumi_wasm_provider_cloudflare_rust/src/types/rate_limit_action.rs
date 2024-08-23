#[derive(serde::Serialize)]
pub struct RateLimitAction {
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    #[serde(rename = "response")]
    pub r#response: Box<Option<crate::types::RateLimitActionResponse>>,
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<i32>>,
}
