#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterClusterConfigGceClusterConfigNodeGroupAffinity {
    /// The URI of a sole-tenant node group resource that the cluster will be created on.
    #[builder(into)]
    #[serde(rename = "nodeGroupUri")]
    pub r#node_group_uri: Box<String>,
}
