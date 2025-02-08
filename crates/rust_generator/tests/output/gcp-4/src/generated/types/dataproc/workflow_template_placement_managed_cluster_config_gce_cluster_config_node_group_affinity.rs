#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkflowTemplatePlacementManagedClusterConfigGceClusterConfigNodeGroupAffinity {
    /// Required. The URI of a sole-tenant /zones/us-central1-a/nodeGroups/node-group-1` * `node-group-1`
    #[builder(into)]
    #[serde(rename = "nodeGroup")]
    pub r#node_group: Box<String>,
}
