#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRule {
    /// Configuration for an issue detection rule.
    #[builder(into, default)]
    #[serde(rename = "issueDetectionConfiguration")]
    pub r#issue_detection_configuration: Box<Option<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleIssueDetectionConfiguration>>,
    /// Configuration for a keyword match rule.
    #[builder(into, default)]
    #[serde(rename = "keywordMatchConfiguration")]
    pub r#keyword_match_configuration: Box<Option<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleKeywordMatchConfiguration>>,
    /// Configuration for a sentiment rule.
    #[builder(into, default)]
    #[serde(rename = "sentimentConfiguration")]
    pub r#sentiment_configuration: Box<Option<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleSentimentConfiguration>>,
    /// Rule type.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}