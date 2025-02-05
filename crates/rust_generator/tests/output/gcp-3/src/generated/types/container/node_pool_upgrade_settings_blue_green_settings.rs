#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NodePoolUpgradeSettingsBlueGreenSettings {
    /// Time needed after draining the entire blue pool.
    /// After this period, the blue pool will be cleaned up.
    #[builder(into, default)]
    #[serde(rename = "nodePoolSoakDuration")]
    pub r#node_pool_soak_duration: Box<Option<String>>,
    /// Specifies the standard policy settings for blue-green upgrades.
    #[builder(into)]
    #[serde(rename = "standardRolloutPolicy")]
    pub r#standard_rollout_policy: Box<super::super::types::container::NodePoolUpgradeSettingsBlueGreenSettingsStandardRolloutPolicy>,
}
