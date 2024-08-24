#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersResponse {
    /// Body content to include in the response.
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    /// HTTP content type to send in the response.
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    /// HTTP status code to send in the response.
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
}
