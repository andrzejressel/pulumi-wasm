#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterConfigurationInfo {
    /// Amazon Resource Name (ARN) of the MSK Configuration to use in the cluster.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// Revision of the MSK Configuration to use in the cluster.
    #[builder(into)]
    #[serde(rename = "revision")]
    pub r#revision: Box<i32>,
}
