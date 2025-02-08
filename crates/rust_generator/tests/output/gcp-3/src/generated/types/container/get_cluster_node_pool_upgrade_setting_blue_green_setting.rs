#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetClusterNodePoolUpgradeSettingBlueGreenSetting {
    /// Time needed after draining entire blue pool. After this period, blue pool will be cleaned up.
    #[builder(into)]
    #[serde(rename = "nodePoolSoakDuration")]
    pub r#node_pool_soak_duration: Box<String>,
    /// Standard rollout policy is the default policy for blue-green.
    #[builder(into)]
    #[serde(rename = "standardRolloutPolicies")]
    pub r#standard_rollout_policies: Box<Vec<super::super::types::container::GetClusterNodePoolUpgradeSettingBlueGreenSettingStandardRolloutPolicy>>,
}
