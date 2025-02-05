#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterPscConfig {
    /// Create an instance that allows connections from Private Service Connect endpoints to the instance.
    #[builder(into, default)]
    #[serde(rename = "pscEnabled")]
    pub r#psc_enabled: Box<Option<bool>>,
}
