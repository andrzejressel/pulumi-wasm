#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterClusterAutoscalingAutoProvisioningDefaultUpgradeSetting {
    /// Settings for blue-green upgrade strategy.
    #[builder(into)]
    #[serde(rename = "blueGreenSettings")]
    pub r#blue_green_settings: Box<Vec<super::super::types::container::GetClusterClusterAutoscalingAutoProvisioningDefaultUpgradeSettingBlueGreenSetting>>,
    /// The maximum number of nodes that can be created beyond the current size of the node pool during the upgrade process.
    #[builder(into)]
    #[serde(rename = "maxSurge")]
    pub r#max_surge: Box<i32>,
    /// The maximum number of nodes that can be simultaneously unavailable during the upgrade process.
    #[builder(into)]
    #[serde(rename = "maxUnavailable")]
    pub r#max_unavailable: Box<i32>,
    /// Update strategy of the node pool.
    #[builder(into)]
    #[serde(rename = "strategy")]
    pub r#strategy: Box<String>,
}
