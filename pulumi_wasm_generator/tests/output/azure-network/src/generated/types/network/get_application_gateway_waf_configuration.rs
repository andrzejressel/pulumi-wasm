#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetApplicationGatewayWafConfiguration {
    /// One or more `disabled_rule_group` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "disabledRuleGroups")]
    pub r#disabled_rule_groups: Box<Vec<super::super::types::network::GetApplicationGatewayWafConfigurationDisabledRuleGroup>>,
    /// Is the Web Application Firewall enabled?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// One or more `exclusion` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Box<Vec<super::super::types::network::GetApplicationGatewayWafConfigurationExclusion>>,
    /// The File Upload Limit in MB.
    #[builder(into)]
    #[serde(rename = "fileUploadLimitMb")]
    pub r#file_upload_limit_mb: Box<i32>,
    /// The Web Application Firewall Mode.
    #[builder(into)]
    #[serde(rename = "firewallMode")]
    pub r#firewall_mode: Box<String>,
    /// The Maximum Request Body Size in KB.
    #[builder(into)]
    #[serde(rename = "maxRequestBodySizeKb")]
    pub r#max_request_body_size_kb: Box<i32>,
    /// Is Request Body Inspection enabled?
    #[builder(into)]
    #[serde(rename = "requestBodyCheck")]
    pub r#request_body_check: Box<bool>,
    /// The Type of the Rule Set used for this Web Application Firewall.
    #[builder(into)]
    #[serde(rename = "ruleSetType")]
    pub r#rule_set_type: Box<String>,
    /// The Version of the Rule Set used for this Web Application Firewall.
    #[builder(into)]
    #[serde(rename = "ruleSetVersion")]
    pub r#rule_set_version: Box<String>,
}
