#[derive(serde::Serialize)]
pub struct RateLimitActionResponse {
    #[serde(rename = "body")]
    pub r#body: Box<String>,
    #[serde(rename = "contentType")]
    pub r#content_type: Box<String>,
}
