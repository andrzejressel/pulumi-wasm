#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationGatewayWafConfiguration {
    /// One or more `disabled_rule_group` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "disabledRuleGroups")]
    pub r#disabled_rule_groups: Box<Option<Vec<super::super::types::network::ApplicationGatewayWafConfigurationDisabledRuleGroup>>>,
    /// Is the Web Application Firewall enabled?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// One or more `exclusion` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Box<Option<Vec<super::super::types::network::ApplicationGatewayWafConfigurationExclusion>>>,
    /// The File Upload Limit in MB. Accepted values are in the range `1`MB to `750`MB for the `WAF_v2` SKU, and `1`MB to `500`MB for all other SKUs. Defaults to `100`MB.
    #[builder(into, default)]
    #[serde(rename = "fileUploadLimitMb")]
    pub r#file_upload_limit_mb: Box<Option<i32>>,
    /// The Web Application Firewall Mode. Possible values are `Detection` and `Prevention`.
    #[builder(into)]
    #[serde(rename = "firewallMode")]
    pub r#firewall_mode: Box<String>,
    /// The Maximum Request Body Size in KB. Accepted values are in the range `1`KB to `128`KB. Defaults to `128`KB.
    #[builder(into, default)]
    #[serde(rename = "maxRequestBodySizeKb")]
    pub r#max_request_body_size_kb: Box<Option<i32>>,
    /// Is Request Body Inspection enabled? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "requestBodyCheck")]
    pub r#request_body_check: Box<Option<bool>>,
    /// The Type of the Rule Set used for this Web Application Firewall. Possible values are `OWASP`, `Microsoft_BotManagerRuleSet` and `Microsoft_DefaultRuleSet`. Defaults to `OWASP`.
    #[builder(into, default)]
    #[serde(rename = "ruleSetType")]
    pub r#rule_set_type: Box<Option<String>>,
    /// The Version of the Rule Set used for this Web Application Firewall. Possible values are `0.1`, `1.0`, `1.1`, `2.1`, `2.2.9`, `3.0`, `3.1` and `3.2`.
    #[builder(into)]
    #[serde(rename = "ruleSetVersion")]
    pub r#rule_set_version: Box<String>,
}
