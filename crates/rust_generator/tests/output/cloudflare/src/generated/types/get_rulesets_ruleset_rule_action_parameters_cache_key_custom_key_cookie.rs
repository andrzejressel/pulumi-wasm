#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyCookie {
    /// List of cookies to check for presence in the custom key.
    #[builder(into, default)]
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Box<Option<Vec<String>>>,
    /// List of cookies to include in the custom key.
    #[builder(into, default)]
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}
