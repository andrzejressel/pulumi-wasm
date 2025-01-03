#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterRebalanceConfig {
    /// The rebalance behavior for the cluster. When not specified, defaults to `NO_REBALANCE`. Possible values: `MODE_UNSPECIFIED`, `NO_REBALANCE`, `AUTO_REBALANCE_ON_SCALE_UP`.
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
}
