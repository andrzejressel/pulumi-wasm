#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersResponse {
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    #[serde(rename = "contentType")]
    pub r#content_type: Box<Option<String>>,
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
}
