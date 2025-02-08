#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyHeader {
    /// List of headers to check for presence in the custom key.
    #[builder(into, default)]
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Box<Option<Vec<String>>>,
    /// Dictionary of headers mapping to lists of values to check for presence in the custom key.
    #[builder(into, default)]
    #[serde(rename = "contains")]
    pub r#contains: Box<Option<std::collections::HashMap<String, Vec<String>>>>,
    /// Exclude the origin header from the custom key.
    #[builder(into, default)]
    #[serde(rename = "excludeOrigin")]
    pub r#exclude_origin: Box<Option<bool>>,
    /// List of headers to include in the custom key.
    #[builder(into, default)]
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}
