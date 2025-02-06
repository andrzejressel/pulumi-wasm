#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetJobDefinitionNodePropertyNodeRangeProperty {
    /// The container details for the node range.
    #[builder(into)]
    #[serde(rename = "containers")]
    pub r#containers: Box<Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainer>>,
    /// The range of nodes, using node index values. A range of 0:3 indicates nodes with index values of 0 through 3. I
    #[builder(into)]
    #[serde(rename = "targetNodes")]
    pub r#target_nodes: Box<String>,
}
