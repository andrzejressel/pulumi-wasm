#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterClusterConfigAuxiliaryNodeGroup {
    /// A node group ID. Generated if not specified. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of from 3 to 33 characters.
    #[builder(into, default)]
    #[serde(rename = "nodeGroupId")]
    pub r#node_group_id: Box<Option<String>>,
    /// Node group configuration.
    #[builder(into)]
    #[serde(rename = "nodeGroups")]
    pub r#node_groups: Box<Vec<super::super::types::dataproc::ClusterClusterConfigAuxiliaryNodeGroupNodeGroup>>,
}
