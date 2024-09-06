#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct PageRuleActionsCacheKeyFieldsHost {
    /// `false` (default) - includes the Host header in the HTTP request sent to the origin; `true` - includes the Host header that was resolved to get the origin IP for the request (e.g. changed with Resolve Override Page Rule).
    #[serde(rename = "resolved")]
    pub r#resolved: Box<Option<bool>>,
}
