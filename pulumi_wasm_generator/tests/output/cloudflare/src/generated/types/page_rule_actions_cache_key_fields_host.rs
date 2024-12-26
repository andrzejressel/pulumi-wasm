#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct PageRuleActionsCacheKeyFieldsHost {
    /// `false` (default) - includes the Host header in the HTTP request sent to the origin; `true` - includes the Host header that was resolved to get the origin IP for the request (e.g. changed with Resolve Override Page Rule).
    #[builder(into, default)]
    #[serde(rename = "resolved")]
    pub r#resolved: Box<Option<bool>>,
}
