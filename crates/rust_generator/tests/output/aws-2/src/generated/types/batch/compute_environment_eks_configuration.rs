#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ComputeEnvironmentEksConfiguration {
    /// The Amazon Resource Name (ARN) of the Amazon EKS cluster.
    #[builder(into)]
    #[serde(rename = "eksClusterArn")]
    pub r#eks_cluster_arn: Box<String>,
    /// The namespace of the Amazon EKS cluster. AWS Batch manages pods in this namespace.
    #[builder(into)]
    #[serde(rename = "kubernetesNamespace")]
    pub r#kubernetes_namespace: Box<String>,
}
