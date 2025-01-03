#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MeshSpecServiceDiscovery {
    /// The IP version to use to control traffic within the mesh. Valid values are `IPv6_PREFERRED`, `IPv4_PREFERRED`, `IPv4_ONLY`, and `IPv6_ONLY`.
    #[builder(into, default)]
    #[serde(rename = "ipPreference")]
    pub r#ip_preference: Box<Option<String>>,
}
