#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionNodeConfig {
    /// Minimum number of nodes in the runtime nodes.
    #[builder(into, default)]
    #[serde(rename = "maxNodeCount")]
    pub r#max_node_count: Box<Option<i32>>,
    /// Minimum number of nodes in the runtime nodes.
    #[builder(into, default)]
    #[serde(rename = "minNodeCount")]
    pub r#min_node_count: Box<Option<i32>>,
}
