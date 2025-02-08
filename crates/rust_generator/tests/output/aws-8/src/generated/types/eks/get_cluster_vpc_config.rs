#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterVpcConfig {
    /// The cluster security group that was created by Amazon EKS for the cluster.
    #[builder(into)]
    #[serde(rename = "clusterSecurityGroupId")]
    pub r#cluster_security_group_id: Box<String>,
    /// Indicates whether or not the Amazon EKS private API server endpoint is enabled.
    #[builder(into)]
    #[serde(rename = "endpointPrivateAccess")]
    pub r#endpoint_private_access: Box<bool>,
    /// Indicates whether or not the Amazon EKS public API server endpoint is enabled.
    #[builder(into)]
    #[serde(rename = "endpointPublicAccess")]
    pub r#endpoint_public_access: Box<bool>,
    /// List of CIDR blocks. Indicates which CIDR blocks can access the Amazon EKS public API server endpoint.
    #[builder(into)]
    #[serde(rename = "publicAccessCidrs")]
    pub r#public_access_cidrs: Box<Vec<String>>,
    /// List of security group IDs
    #[builder(into)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Box<Vec<String>>,
    /// List of subnet IDs
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Box<Vec<String>>,
    /// The VPC associated with your cluster.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Box<String>,
}
