#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsRulesetRuleActionParametersResponse {
    /// Body content to include in the response.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    /// HTTP content type to send in the response.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    /// HTTP status code to send in the response.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
}
