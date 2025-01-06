#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPrivateCloudManagementClusterNodeTypeConfig {
    /// Customized number of cores available to each node of the type.
    /// This number must always be one of 'nodeType.availableCustomCoreCounts'.
    /// If zero is provided max value from 'nodeType.availableCustomCoreCounts' will be used.
    /// This cannot be changed once the PrivateCloud is created.
    #[builder(into)]
    #[serde(rename = "customCoreCount")]
    pub r#custom_core_count: Box<i32>,
    /// The number of nodes of this type in the cluster.
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: Box<i32>,
    #[builder(into)]
    #[serde(rename = "nodeTypeId")]
    pub r#node_type_id: Box<String>,
}
