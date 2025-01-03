#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SecurityPolicyAdvancedOptionsConfig {
    /// Custom configuration to apply the JSON parsing. Only applicable when
    /// `json_parsing` is set to `STANDARD`. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "jsonCustomConfig")]
    pub r#json_custom_config: Box<Option<super::super::types::compute::SecurityPolicyAdvancedOptionsConfigJsonCustomConfig>>,
    /// Whether or not to JSON parse the payload body. Defaults to `DISABLED`.
    /// * `DISABLED` - Don't parse JSON payloads in POST bodies.
    /// * `STANDARD` - Parse JSON payloads in POST bodies.
    /// * `STANDARD_WITH_GRAPHQL` - Parse JSON and GraphQL payloads in POST bodies.
    #[builder(into, default)]
    #[serde(rename = "jsonParsing")]
    pub r#json_parsing: Box<Option<String>>,
    /// Log level to use. Defaults to `NORMAL`.
    /// * `NORMAL` - Normal log level.
    /// * `VERBOSE` - Verbose log level.
    #[builder(into, default)]
    #[serde(rename = "logLevel")]
    pub r#log_level: Box<Option<String>>,
    /// An optional list of case-insensitive request header names to use for resolving the callers client IP address.
    #[builder(into, default)]
    #[serde(rename = "userIpRequestHeaders")]
    pub r#user_ip_request_headers: Box<Option<Vec<String>>>,
}
