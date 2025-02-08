#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct V2QueuedResourceTpuNodeSpec {
    /// The node.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "node")]
    pub r#node: Box<super::super::types::tpu::V2QueuedResourceTpuNodeSpecNode>,
    /// Unqualified node identifier used to identify the node in the project once provisioned.
    #[builder(into, default)]
    #[serde(rename = "nodeId")]
    pub r#node_id: Box<Option<String>>,
    /// The parent resource name.
    #[builder(into)]
    #[serde(rename = "parent")]
    pub r#parent: Box<String>,
}
