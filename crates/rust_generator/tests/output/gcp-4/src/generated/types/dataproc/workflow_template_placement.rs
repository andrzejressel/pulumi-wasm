#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WorkflowTemplatePlacement {
    /// A selector that chooses target cluster for jobs based on metadata. The selector is evaluated at the time each job is submitted.
    #[builder(into, default)]
    #[serde(rename = "clusterSelector")]
    pub r#cluster_selector: Box<Option<super::super::types::dataproc::WorkflowTemplatePlacementClusterSelector>>,
    /// A cluster that is managed by the workflow.
    #[builder(into, default)]
    #[serde(rename = "managedCluster")]
    pub r#managed_cluster: Box<Option<super::super::types::dataproc::WorkflowTemplatePlacementManagedCluster>>,
}
