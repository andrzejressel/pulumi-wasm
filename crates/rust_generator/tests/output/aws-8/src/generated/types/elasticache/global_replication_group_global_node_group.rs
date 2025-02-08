#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GlobalReplicationGroupGlobalNodeGroup {
    /// The ID of the global node group.
    #[builder(into, default)]
    #[serde(rename = "globalNodeGroupId")]
    pub r#global_node_group_id: Box<Option<String>>,
    /// The keyspace for this node group.
    #[builder(into, default)]
    #[serde(rename = "slots")]
    pub r#slots: Box<Option<String>>,
}
