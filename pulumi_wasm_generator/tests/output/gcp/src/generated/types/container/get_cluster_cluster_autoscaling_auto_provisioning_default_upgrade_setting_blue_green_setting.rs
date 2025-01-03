#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterClusterAutoscalingAutoProvisioningDefaultUpgradeSettingBlueGreenSetting {
    /// Time needed after draining entire blue pool. After this period, blue pool will be cleaned up.
    /// 
    /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "nodePoolSoakDuration")]
    pub r#node_pool_soak_duration: Box<String>,
    /// Standard policy for the blue-green upgrade.
    #[builder(into)]
    #[serde(rename = "standardRolloutPolicies")]
    pub r#standard_rollout_policies: Box<Vec<super::super::types::container::GetClusterClusterAutoscalingAutoProvisioningDefaultUpgradeSettingBlueGreenSettingStandardRolloutPolicy>>,
}
