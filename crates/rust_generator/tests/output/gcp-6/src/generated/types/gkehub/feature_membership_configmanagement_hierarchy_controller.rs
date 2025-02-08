#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureMembershipConfigmanagementHierarchyController {
    /// Whether hierarchical resource quota is enabled in this cluster.
    #[builder(into, default)]
    #[serde(rename = "enableHierarchicalResourceQuota")]
    pub r#enable_hierarchical_resource_quota: Box<Option<bool>>,
    /// Whether pod tree labels are enabled in this cluster.
    #[builder(into, default)]
    #[serde(rename = "enablePodTreeLabels")]
    pub r#enable_pod_tree_labels: Box<Option<bool>>,
    /// Whether Hierarchy Controller is enabled in this cluster.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
}
