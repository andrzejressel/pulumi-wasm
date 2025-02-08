#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetClusterNodePoolUpgradeSetting {
    /// Settings for BlueGreen node pool upgrade.
    #[builder(into)]
    #[serde(rename = "blueGreenSettings")]
    pub r#blue_green_settings: Box<Vec<super::super::types::container::GetClusterNodePoolUpgradeSettingBlueGreenSetting>>,
    /// The number of additional nodes that can be added to the node pool during an upgrade. Increasing max_surge raises the number of nodes that can be upgraded simultaneously. Can be set to 0 or greater.
    #[builder(into)]
    #[serde(rename = "maxSurge")]
    pub r#max_surge: Box<i32>,
    /// The number of nodes that can be simultaneously unavailable during an upgrade. Increasing max_unavailable raises the number of nodes that can be upgraded in parallel. Can be set to 0 or greater.
    #[builder(into)]
    #[serde(rename = "maxUnavailable")]
    pub r#max_unavailable: Box<i32>,
    /// Update strategy for the given nodepool.
    #[builder(into)]
    #[serde(rename = "strategy")]
    pub r#strategy: Box<String>,
}
