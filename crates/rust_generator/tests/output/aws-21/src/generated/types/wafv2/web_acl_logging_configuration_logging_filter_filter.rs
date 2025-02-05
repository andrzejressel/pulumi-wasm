#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclLoggingConfigurationLoggingFilterFilter {
    /// Parameter that determines how to handle logs that meet the conditions and requirements of the filter. The valid values for `behavior` are `KEEP` or `DROP`.
    #[builder(into)]
    #[serde(rename = "behavior")]
    pub r#behavior: Box<String>,
    /// Match condition(s) for the filter. See Condition below for more details.
    #[builder(into)]
    #[serde(rename = "conditions")]
    pub r#conditions: Box<Vec<super::super::types::wafv2::WebAclLoggingConfigurationLoggingFilterFilterCondition>>,
    /// Logic to apply to the filtering conditions. You can specify that a log must match all conditions or at least one condition in order to satisfy the filter. Valid values for `requirement` are `MEETS_ALL` or `MEETS_ANY`.
    #[builder(into)]
    #[serde(rename = "requirement")]
    pub r#requirement: Box<String>,
}
