#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct LaunchTemplatePlacement {
    /// The affinity setting for an instance on a Dedicated Host.
    #[builder(into, default)]
    #[serde(rename = "affinity")]
    pub r#affinity: Box<Option<String>>,
    /// The Availability Zone for the instance.
    #[builder(into, default)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Box<Option<String>>,
    /// The name of the placement group for the instance.
    #[builder(into, default)]
    #[serde(rename = "groupName")]
    pub r#group_name: Box<Option<String>>,
    /// The ID of the Dedicated Host for the instance.
    #[builder(into, default)]
    #[serde(rename = "hostId")]
    pub r#host_id: Box<Option<String>>,
    /// The ARN of the Host Resource Group in which to launch instances.
    #[builder(into, default)]
    #[serde(rename = "hostResourceGroupArn")]
    pub r#host_resource_group_arn: Box<Option<String>>,
    /// The number of the partition the instance should launch in. Valid only if the placement group strategy is set to partition.
    #[builder(into, default)]
    #[serde(rename = "partitionNumber")]
    pub r#partition_number: Box<Option<i32>>,
    /// Reserved for future use.
    #[builder(into, default)]
    #[serde(rename = "spreadDomain")]
    pub r#spread_domain: Box<Option<String>>,
    /// The tenancy of the instance (if the instance is running in a VPC). Can be `default`, `dedicated`, or `host`.
    #[builder(into, default)]
    #[serde(rename = "tenancy")]
    pub r#tenancy: Box<Option<String>>,
}
