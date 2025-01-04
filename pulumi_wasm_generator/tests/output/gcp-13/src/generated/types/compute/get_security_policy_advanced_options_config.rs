#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSecurityPolicyAdvancedOptionsConfig {
    /// Custom configuration to apply the JSON parsing. Only applicable when JSON parsing is set to STANDARD.
    #[builder(into)]
    #[serde(rename = "jsonCustomConfigs")]
    pub r#json_custom_configs: Box<Vec<super::super::types::compute::GetSecurityPolicyAdvancedOptionsConfigJsonCustomConfig>>,
    /// JSON body parsing. Supported values include: "DISABLED", "STANDARD".
    #[builder(into)]
    #[serde(rename = "jsonParsing")]
    pub r#json_parsing: Box<String>,
    /// Logging level. Supported values include: "NORMAL", "VERBOSE".
    #[builder(into)]
    #[serde(rename = "logLevel")]
    pub r#log_level: Box<String>,
    /// An optional list of case-insensitive request header names to use for resolving the callers client IP address.
    #[builder(into)]
    #[serde(rename = "userIpRequestHeaders")]
    pub r#user_ip_request_headers: Box<Vec<String>>,
}
