#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AwsClusterControlPlane {
    /// Authentication configuration for management of AWS resources.
    #[builder(into)]
    #[serde(rename = "awsServicesAuthentication")]
    pub r#aws_services_authentication: Box<super::super::types::container::AwsClusterControlPlaneAwsServicesAuthentication>,
    /// The ARN of the AWS KMS key used to encrypt cluster configuration.
    #[builder(into)]
    #[serde(rename = "configEncryption")]
    pub r#config_encryption: Box<super::super::types::container::AwsClusterControlPlaneConfigEncryption>,
    /// The ARN of the AWS KMS key used to encrypt cluster secrets.
    #[builder(into)]
    #[serde(rename = "databaseEncryption")]
    pub r#database_encryption: Box<super::super::types::container::AwsClusterControlPlaneDatabaseEncryption>,
    /// The name of the AWS IAM instance pofile to assign to each control plane replica.
    #[builder(into)]
    #[serde(rename = "iamInstanceProfile")]
    pub r#iam_instance_profile: Box<String>,
    /// Details of placement information for an instance.
    #[builder(into, default)]
    #[serde(rename = "instancePlacement")]
    pub r#instance_placement: Box<Option<super::super::types::container::AwsClusterControlPlaneInstancePlacement>>,
    /// Optional. The AWS instance type. When unspecified, it defaults to `m5.large`.
    #[builder(into, default)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Box<Option<String>>,
    /// Optional. Configuration related to the main volume provisioned for each control plane replica. The main volume is in charge of storing all of the cluster's etcd state. Volumes will be provisioned in the availability zone associated with the corresponding subnet. When unspecified, it defaults to 8 GiB with the GP2 volume type.
    #[builder(into, default)]
    #[serde(rename = "mainVolume")]
    pub r#main_volume: Box<Option<super::super::types::container::AwsClusterControlPlaneMainVolume>>,
    /// Proxy configuration for outbound HTTP(S) traffic.
    #[builder(into, default)]
    #[serde(rename = "proxyConfig")]
    pub r#proxy_config: Box<Option<super::super::types::container::AwsClusterControlPlaneProxyConfig>>,
    /// Optional. Configuration related to the root volume provisioned for each control plane replica. Volumes will be provisioned in the availability zone associated with the corresponding subnet. When unspecified, it defaults to 32 GiB with the GP2 volume type.
    #[builder(into, default)]
    #[serde(rename = "rootVolume")]
    pub r#root_volume: Box<Option<super::super::types::container::AwsClusterControlPlaneRootVolume>>,
    /// Optional. The IDs of additional security groups to add to control plane replicas. The Anthos Multi-Cloud API will automatically create and manage security groups with the minimum rules needed for a functioning cluster.
    #[builder(into, default)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Box<Option<Vec<String>>>,
    /// Optional. SSH configuration for how to access the underlying control plane machines.
    #[builder(into, default)]
    #[serde(rename = "sshConfig")]
    pub r#ssh_config: Box<Option<super::super::types::container::AwsClusterControlPlaneSshConfig>>,
    /// The list of subnets where control plane replicas will run. A replica will be provisioned on each subnet and up to three values can be provided. Each subnet must be in a different AWS Availability Zone (AZ).
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Box<Vec<String>>,
    /// Optional. A set of AWS resource tags to propagate to all underlying managed AWS resources. Specify at most 50 pairs containing alphanumerics, spaces, and symbols (.+-=_:@/). Keys can be up to 127 Unicode characters. Values can be up to 255 Unicode characters.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<std::collections::HashMap<String, String>>>,
    /// The Kubernetes version to run on control plane replicas (e.g. `1.19.10-gke.1000`). You can list all supported versions on a given Google Cloud region by calling .
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
