#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct RateLimitActionResponse {
    /// The body to return, the content here should conform to the `content_type`.
    #[builder(into)]
    #[serde(rename = "body")]
    pub r#body: Box<String>,
    /// The content-type of the body. Available values: `text/plain`, `text/xml`, `application/json`.
    #[builder(into)]
    #[serde(rename = "contentType")]
    pub r#content_type: Box<String>,
}
