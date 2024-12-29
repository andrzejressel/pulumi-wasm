#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ReplicatorKafkaClusterAmazonMskCluster {
    /// The ARN of an Amazon MSK cluster.
    #[builder(into)]
    #[serde(rename = "mskClusterArn")]
    pub r#msk_cluster_arn: Box<String>,
}
