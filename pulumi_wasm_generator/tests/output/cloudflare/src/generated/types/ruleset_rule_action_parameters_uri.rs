#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RulesetRuleActionParametersUri {
    #[builder(into, default)]
    #[serde(rename = "origin")]
    pub r#origin: Box<Option<bool>>,
    /// URI path configuration when performing a URL rewrite.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<super::types::RulesetRuleActionParametersUriPath>>,
    /// Query string configuration when performing a URL rewrite.
    #[builder(into, default)]
    #[serde(rename = "query")]
    pub r#query: Box<Option<super::types::RulesetRuleActionParametersUriQuery>>,
}
