#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterSecondaryConfig {
    /// Name of the primary cluster must be in the format
    /// 'projects/{project}/locations/{location}/clusters/{cluster_id}'
    #[builder(into)]
    #[serde(rename = "primaryClusterName")]
    pub r#primary_cluster_name: Box<String>,
}
