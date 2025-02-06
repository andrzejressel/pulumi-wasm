#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetEnvironmentConfigWebServerNetworkAccessControl {
    /// A collection of allowed IP ranges with descriptions.
    #[builder(into)]
    #[serde(rename = "allowedIpRanges")]
    pub r#allowed_ip_ranges: Box<Vec<super::super::types::composer::GetEnvironmentConfigWebServerNetworkAccessControlAllowedIpRange>>,
}
