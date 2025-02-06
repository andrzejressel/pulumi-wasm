#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualNodeSpecServiceDiscoveryDn {
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Box<String>,
    #[builder(into)]
    #[serde(rename = "ipPreference")]
    pub r#ip_preference: Box<String>,
    #[builder(into)]
    #[serde(rename = "responseType")]
    pub r#response_type: Box<String>,
}
