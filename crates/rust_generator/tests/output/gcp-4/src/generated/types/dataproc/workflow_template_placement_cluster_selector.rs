#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WorkflowTemplatePlacementClusterSelector {
    /// Required. The cluster labels. Cluster must have all labels to match.
    #[builder(into)]
    #[serde(rename = "clusterLabels")]
    pub r#cluster_labels: Box<std::collections::HashMap<String, String>>,
    /// The zone where workflow process executes. This parameter does not affect the selection of the cluster. If unspecified, the zone of the first cluster matching the selector is used.
    #[builder(into, default)]
    #[serde(rename = "zone")]
    pub r#zone: Box<Option<String>>,
}
