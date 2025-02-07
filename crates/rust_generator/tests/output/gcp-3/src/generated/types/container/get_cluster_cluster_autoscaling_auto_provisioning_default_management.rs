#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterClusterAutoscalingAutoProvisioningDefaultManagement {
    /// Specifies whether the node auto-repair is enabled for the node pool. If enabled, the nodes in this node pool will be monitored and, if they fail health checks too many times, an automatic repair action will be triggered.
    #[builder(into)]
    #[serde(rename = "autoRepair")]
    pub r#auto_repair: Box<bool>,
    /// Specifies whether node auto-upgrade is enabled for the node pool. If enabled, node auto-upgrade helps keep the nodes in your node pool up to date with the latest release version of Kubernetes.
    #[builder(into)]
    #[serde(rename = "autoUpgrade")]
    pub r#auto_upgrade: Box<bool>,
    /// Specifies the Auto Upgrade knobs for the node pool.
    #[builder(into)]
    #[serde(rename = "upgradeOptions")]
    pub r#upgrade_options: Box<Vec<super::super::types::container::GetClusterClusterAutoscalingAutoProvisioningDefaultManagementUpgradeOption>>,
}
