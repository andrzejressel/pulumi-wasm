#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleKeywordMatchConfiguration {
    /// Collection of keywords to match.
    #[builder(into)]
    #[serde(rename = "keywords")]
    pub r#keywords: Box<Vec<String>>,
    /// Negate the rule.
    #[builder(into, default)]
    #[serde(rename = "negate")]
    pub r#negate: Box<Option<bool>>,
    /// Rule name.
    #[builder(into)]
    #[serde(rename = "ruleName")]
    pub r#rule_name: Box<String>,
}
