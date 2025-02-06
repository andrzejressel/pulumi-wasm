#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclLoggingConfigurationLoggingFilter {
    /// Default handling for logs that don't match any of the specified filtering conditions. Valid values for `default_behavior` are `KEEP` or `DROP`.
    #[builder(into)]
    #[serde(rename = "defaultBehavior")]
    pub r#default_behavior: Box<String>,
    /// Filter(s) that you want to apply to the logs. See Filter below for more details.
    #[builder(into)]
    #[serde(rename = "filters")]
    pub r#filters: Box<Vec<super::super::types::wafv2::WebAclLoggingConfigurationLoggingFilterFilter>>,
}
