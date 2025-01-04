#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterNodePoolDefaults {
    /// Subset of NodeConfig message that has defaults.
    #[builder(into, default)]
    #[serde(rename = "nodeConfigDefaults")]
    pub r#node_config_defaults: Box<Option<super::super::types::container::ClusterNodePoolDefaultsNodeConfigDefaults>>,
}
