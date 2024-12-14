#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct PageRuleActionsForwardingUrl {
    /// The status code to use for the redirection.
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<i32>,
    /// The URL to which the page rule should forward.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
}
