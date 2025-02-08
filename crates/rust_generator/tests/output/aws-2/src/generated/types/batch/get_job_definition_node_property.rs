#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetJobDefinitionNodeProperty {
    /// Specifies the node index for the main node of a multi-node parallel job. This node index value must be fewer than the number of nodes.
    #[builder(into)]
    #[serde(rename = "mainNode")]
    pub r#main_node: Box<i32>,
    /// A list of node ranges and their properties that are associated with a multi-node parallel job.
    #[builder(into)]
    #[serde(rename = "nodeRangeProperties")]
    pub r#node_range_properties: Box<Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangeProperty>>,
    /// The number of nodes that are associated with a multi-node parallel job.
    #[builder(into)]
    #[serde(rename = "numNodes")]
    pub r#num_nodes: Box<i32>,
}
