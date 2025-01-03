#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainConfigurationTlsConfig {
    /// The security policy for a domain configuration.
    #[builder(into, default)]
    #[serde(rename = "securityPolicy")]
    pub r#security_policy: Box<Option<String>>,
}
