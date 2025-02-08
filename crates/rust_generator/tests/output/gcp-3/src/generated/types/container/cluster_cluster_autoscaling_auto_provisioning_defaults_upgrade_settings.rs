#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterClusterAutoscalingAutoProvisioningDefaultsUpgradeSettings {
    /// Settings for blue-green upgrade strategy. To be specified when strategy is set to BLUE_GREEN. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "blueGreenSettings")]
    pub r#blue_green_settings: Box<Option<super::super::types::container::ClusterClusterAutoscalingAutoProvisioningDefaultsUpgradeSettingsBlueGreenSettings>>,
    /// The maximum number of nodes that can be created beyond the current size of the node pool during the upgrade process. To be used when strategy is set to SURGE. Default is 0.
    #[builder(into, default)]
    #[serde(rename = "maxSurge")]
    pub r#max_surge: Box<Option<i32>>,
    /// The maximum number of nodes that can be simultaneously unavailable during the upgrade process. To be used when strategy is set to SURGE. Default is 0.
    #[builder(into, default)]
    #[serde(rename = "maxUnavailable")]
    pub r#max_unavailable: Box<Option<i32>>,
    /// Strategy used for node pool update. Strategy can only be one of BLUE_GREEN or SURGE. The default is value is SURGE.
    #[builder(into, default)]
    #[serde(rename = "strategy")]
    pub r#strategy: Box<Option<String>>,
}
