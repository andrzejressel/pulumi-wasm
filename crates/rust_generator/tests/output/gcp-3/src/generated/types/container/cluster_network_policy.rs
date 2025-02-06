#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterNetworkPolicy {
    /// Whether network policy is enabled on the cluster.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// The selected network policy provider. Defaults to PROVIDER_UNSPECIFIED.
    #[builder(into, default)]
    #[serde(rename = "provider")]
    pub r#provider: Box<Option<String>>,
}
