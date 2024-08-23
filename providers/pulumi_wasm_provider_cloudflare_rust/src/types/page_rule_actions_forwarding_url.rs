#[derive(serde::Serialize)]
pub struct PageRuleActionsForwardingUrl {
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<i32>,
    #[serde(rename = "url")]
    pub r#url: Box<String>,
}
