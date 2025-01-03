#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GlobalClusterGlobalClusterMember {
    /// Amazon Resource Name (ARN) of member DB Cluster.
    #[builder(into, default)]
    #[serde(rename = "dbClusterArn")]
    pub r#db_cluster_arn: Box<Option<String>>,
    /// Whether the member is the primary DB Cluster.
    #[builder(into, default)]
    #[serde(rename = "isWriter")]
    pub r#is_writer: Box<Option<bool>>,
}
