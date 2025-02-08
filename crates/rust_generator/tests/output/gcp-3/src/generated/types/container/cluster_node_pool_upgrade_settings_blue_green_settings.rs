#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterNodePoolUpgradeSettingsBlueGreenSettings {
    /// Time needed after draining entire blue pool. After this period, blue pool will be cleaned up. A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
    #[builder(into, default)]
    #[serde(rename = "nodePoolSoakDuration")]
    pub r#node_pool_soak_duration: Box<Option<String>>,
    /// Standard policy for the blue-green upgrade. To be specified when strategy is set to BLUE_GREEN. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "standardRolloutPolicy")]
    pub r#standard_rollout_policy: Box<super::super::types::container::ClusterNodePoolUpgradeSettingsBlueGreenSettingsStandardRolloutPolicy>,
}
