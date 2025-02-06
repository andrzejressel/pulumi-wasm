#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterClusterConfigGceClusterConfigNodeGroupAffinity {
    /// The URI of a sole-tenant node group resource that the cluster will be created on.
    #[builder(into)]
    #[serde(rename = "nodeGroupUri")]
    pub r#node_group_uri: Box<String>,
}
