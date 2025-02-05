#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterVpcConfig {
    /// Cluster security group that was created by Amazon EKS for the cluster. Managed node groups use this security group for control-plane-to-data-plane communication.
    #[builder(into, default)]
    #[serde(rename = "clusterSecurityGroupId")]
    pub r#cluster_security_group_id: Box<Option<String>>,
    /// Whether the Amazon EKS private API server endpoint is enabled. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "endpointPrivateAccess")]
    pub r#endpoint_private_access: Box<Option<bool>>,
    /// Whether the Amazon EKS public API server endpoint is enabled. Default is `true`.
    #[builder(into, default)]
    #[serde(rename = "endpointPublicAccess")]
    pub r#endpoint_public_access: Box<Option<bool>>,
    /// List of CIDR blocks. Indicates which CIDR blocks can access the Amazon EKS public API server endpoint when enabled. EKS defaults this to a list with `0.0.0.0/0`. The provider will only perform drift detection of its value when present in a configuration.
    #[builder(into, default)]
    #[serde(rename = "publicAccessCidrs")]
    pub r#public_access_cidrs: Box<Option<Vec<String>>>,
    /// List of security group IDs for the cross-account elastic network interfaces that Amazon EKS creates to use to allow communication between your worker nodes and the Kubernetes control plane.
    #[builder(into, default)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Box<Option<Vec<String>>>,
    /// List of subnet IDs. Must be in at least two different availability zones. Amazon EKS creates cross-account elastic network interfaces in these subnets to allow communication between your worker nodes and the Kubernetes control plane.
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Box<Vec<String>>,
    /// ID of the VPC associated with your cluster.
    #[builder(into, default)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Box<Option<String>>,
}
