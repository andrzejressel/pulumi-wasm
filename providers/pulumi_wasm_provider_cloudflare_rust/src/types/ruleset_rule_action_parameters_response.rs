#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct RulesetRuleActionParametersResponse {
    /// Content of the custom error response.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    /// Content-Type of the custom error response.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    /// Status code for which the edge TTL is applied.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
}
