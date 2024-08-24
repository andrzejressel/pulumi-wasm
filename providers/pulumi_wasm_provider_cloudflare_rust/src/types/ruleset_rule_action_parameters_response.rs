#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersResponse {
    /// Content of the custom error response.
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    /// Content-Type of the custom error response.
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    /// Status code for which the edge TTL is applied.
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
}
