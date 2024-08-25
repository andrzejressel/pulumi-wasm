#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct PageRuleActionsForwardingUrl {
    /// The status code to use for the redirection.
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<i32>,
    /// The URL to which the page rule should forward.
    #[serde(rename = "url")]
    pub r#url: Box<String>,
}
