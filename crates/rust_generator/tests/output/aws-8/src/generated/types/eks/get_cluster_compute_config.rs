#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterComputeConfig {
    /// Whether zonal shift is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// List of node pools for the EKS Auto Mode compute capability.
    #[builder(into)]
    #[serde(rename = "nodePools")]
    pub r#node_pools: Box<Vec<String>>,
    /// The ARN of the IAM Role EKS will assign to EC2 Managed Instances in your EKS Auto Mode cluster.
    #[builder(into)]
    #[serde(rename = "nodeRoleArn")]
    pub r#node_role_arn: Box<String>,
}
