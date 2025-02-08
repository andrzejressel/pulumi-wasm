#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterNodePoolManagement {
    /// Specifies whether the node auto-repair is enabled for the node pool. If enabled, the nodes in this node pool will be monitored and, if they fail health checks too many times, an automatic repair action will be triggered.
    /// 
    /// This block also contains several computed attributes, documented below.
    #[builder(into, default)]
    #[serde(rename = "autoRepair")]
    pub r#auto_repair: Box<Option<bool>>,
    /// Specifies whether node auto-upgrade is enabled for the node pool. If enabled, node auto-upgrade helps keep the nodes in your node pool up to date with the latest release version of Kubernetes.
    #[builder(into, default)]
    #[serde(rename = "autoUpgrade")]
    pub r#auto_upgrade: Box<Option<bool>>,
}
