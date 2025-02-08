#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterNodeTypeConfig {
    /// Customized number of cores available to each node of the type.
    /// This number must always be one of `nodeType.availableCustomCoreCounts`.
    /// If zero is provided max value from `nodeType.availableCustomCoreCounts` will be used.
    /// Once the customer is created then corecount cannot be changed.
    #[builder(into, default)]
    #[serde(rename = "customCoreCount")]
    pub r#custom_core_count: Box<Option<i32>>,
    /// The number of nodes of this type in the cluster.
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: Box<i32>,
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "nodeTypeId")]
    pub r#node_type_id: Box<String>,
}
