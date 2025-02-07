#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AwsNodePoolConfig {
    /// Optional. Configuration related to CloudWatch metrics collection on the Auto Scaling group of the node pool. When unspecified, metrics collection is disabled.
    #[builder(into, default)]
    #[serde(rename = "autoscalingMetricsCollection")]
    pub r#autoscaling_metrics_collection: Box<Option<super::super::types::container::AwsNodePoolConfigAutoscalingMetricsCollection>>,
    /// The ARN of the AWS KMS key used to encrypt node pool configuration.
    #[builder(into)]
    #[serde(rename = "configEncryption")]
    pub r#config_encryption: Box<super::super::types::container::AwsNodePoolConfigConfigEncryption>,
    /// The name of the AWS IAM role assigned to nodes in the pool.
    #[builder(into)]
    #[serde(rename = "iamInstanceProfile")]
    pub r#iam_instance_profile: Box<String>,
    /// The OS image type to use on node pool instances.
    #[builder(into, default)]
    #[serde(rename = "imageType")]
    pub r#image_type: Box<Option<String>>,
    /// Details of placement information for an instance.
    #[builder(into, default)]
    #[serde(rename = "instancePlacement")]
    pub r#instance_placement: Box<Option<super::super::types::container::AwsNodePoolConfigInstancePlacement>>,
    /// Optional. The AWS instance type. When unspecified, it defaults to `m5.large`.
    #[builder(into, default)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Box<Option<String>>,
    /// Optional. The initial labels assigned to nodes of this node pool. An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// Proxy configuration for outbound HTTP(S) traffic.
    #[builder(into, default)]
    #[serde(rename = "proxyConfig")]
    pub r#proxy_config: Box<Option<super::super::types::container::AwsNodePoolConfigProxyConfig>>,
    /// Optional. Template for the root volume provisioned for node pool nodes. Volumes will be provisioned in the availability zone assigned to the node pool subnet. When unspecified, it defaults to 32 GiB with the GP2 volume type.
    #[builder(into, default)]
    #[serde(rename = "rootVolume")]
    pub r#root_volume: Box<Option<super::super::types::container::AwsNodePoolConfigRootVolume>>,
    /// Optional. The IDs of additional security groups to add to nodes in this pool. The manager will automatically create security groups with minimum rules needed for a functioning cluster.
    #[builder(into, default)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Box<Option<Vec<String>>>,
    /// Optional. When specified, the node pool will provision Spot instances from the set of spot_config.instance_types. This field is mutually exclusive with `instance_type`
    #[builder(into, default)]
    #[serde(rename = "spotConfig")]
    pub r#spot_config: Box<Option<super::super::types::container::AwsNodePoolConfigSpotConfig>>,
    /// Optional. The SSH configuration.
    #[builder(into, default)]
    #[serde(rename = "sshConfig")]
    pub r#ssh_config: Box<Option<super::super::types::container::AwsNodePoolConfigSshConfig>>,
    /// Optional. Key/value metadata to assign to each underlying AWS resource. Specify at most 50 pairs containing alphanumerics, spaces, and symbols (.+-=_:@/). Keys can be up to 127 Unicode characters. Values can be up to 255 Unicode characters.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<std::collections::HashMap<String, String>>>,
    /// Optional. The initial taints assigned to nodes of this node pool.
    #[builder(into, default)]
    #[serde(rename = "taints")]
    pub r#taints: Box<Option<Vec<super::super::types::container::AwsNodePoolConfigTaint>>>,
}
