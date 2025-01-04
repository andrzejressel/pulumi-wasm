#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FhirServiceAuthentication {
    /// The intended audience to receive authentication tokens for the service.
    #[builder(into)]
    #[serde(rename = "audience")]
    pub r#audience: Box<String>,
    #[builder(into)]
    #[serde(rename = "authority")]
    pub r#authority: Box<String>,
    /// Whether smart proxy is enabled.
    #[builder(into, default)]
    #[serde(rename = "smartProxyEnabled")]
    pub r#smart_proxy_enabled: Box<Option<bool>>,
}
