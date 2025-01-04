#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterComputeConfig {
    /// Request to enable or disable the compute capability on your EKS Auto Mode cluster. If the compute capability is enabled, EKS Auto Mode will create and delete EC2 Managed Instances in your Amazon Web Services account.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Configuration for node pools that defines the compute resources for your EKS Auto Mode cluster. Valid options are `general-purpose` and `system`.
    #[builder(into, default)]
    #[serde(rename = "nodePools")]
    pub r#node_pools: Box<Option<Vec<String>>>,
    /// The ARN of the IAM Role EKS will assign to EC2 Managed Instances in your EKS Auto Mode cluster. This value cannot be changed after the compute capability of EKS Auto Mode is enabled..
    #[builder(into, default)]
    #[serde(rename = "nodeRoleArn")]
    pub r#node_role_arn: Box<Option<String>>,
}
