#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct RulesetRuleActionParametersUri {
    /// List of properties to change request origin.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "origin")]
    pub r#origin: Box<Option<bool>>,
    /// URI path configuration when performing a URL rewrite.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "path")]
    pub r#path: Box<Option<crate::types::RulesetRuleActionParametersUriPath>>,
    /// Query string configuration when performing a URL rewrite.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "query")]
    pub r#query: Box<Option<crate::types::RulesetRuleActionParametersUriQuery>>,
}
