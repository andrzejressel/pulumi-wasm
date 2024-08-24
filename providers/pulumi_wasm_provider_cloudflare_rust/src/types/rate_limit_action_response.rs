#[derive(serde::Serialize)]
pub struct RateLimitActionResponse {
    /// The body to return, the content here should conform to the `content_type`.
    #[serde(rename = "body")]
    pub r#body: Box<String>,
    /// The content-type of the body. Available values: `text/plain`, `text/xml`, `application/json`.
    #[serde(rename = "contentType")]
    pub r#content_type: Box<String>,
}
