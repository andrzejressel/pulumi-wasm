#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpotFleetRequestLaunchSpecification {
    #[builder(into)]
    #[serde(rename = "ami")]
    pub r#ami: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "associatePublicIpAddress")]
    pub r#associate_public_ip_address: Box<Option<bool>>,
    /// The availability zone in which to place the request.
    #[builder(into, default)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "ebsBlockDevices")]
    pub r#ebs_block_devices: Box<Option<Vec<super::super::types::ec2::SpotFleetRequestLaunchSpecificationEbsBlockDevice>>>,
    #[builder(into, default)]
    #[serde(rename = "ebsOptimized")]
    pub r#ebs_optimized: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "ephemeralBlockDevices")]
    pub r#ephemeral_block_devices: Box<Option<Vec<super::super::types::ec2::SpotFleetRequestLaunchSpecificationEphemeralBlockDevice>>>,
    #[builder(into, default)]
    #[serde(rename = "iamInstanceProfile")]
    pub r#iam_instance_profile: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "iamInstanceProfileArn")]
    pub r#iam_instance_profile_arn: Box<Option<String>>,
    /// The type of instance to request.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "keyName")]
    pub r#key_name: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "monitoring")]
    pub r#monitoring: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "placementGroup")]
    pub r#placement_group: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "placementTenancy")]
    pub r#placement_tenancy: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "rootBlockDevices")]
    pub r#root_block_devices: Box<Option<Vec<super::super::types::ec2::SpotFleetRequestLaunchSpecificationRootBlockDevice>>>,
    /// The maximum bid price per unit hour.
    #[builder(into, default)]
    #[serde(rename = "spotPrice")]
    pub r#spot_price: Box<Option<String>>,
    /// The subnet in which to launch the requested instance.
    #[builder(into, default)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<Option<String>>,
    /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<std::collections::HashMap<String, String>>>,
    #[builder(into, default)]
    #[serde(rename = "userData")]
    pub r#user_data: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "vpcSecurityGroupIds")]
    pub r#vpc_security_group_ids: Box<Option<Vec<String>>>,
    /// The capacity added to the fleet by a fulfilled request.
    #[builder(into, default)]
    #[serde(rename = "weightedCapacity")]
    pub r#weighted_capacity: Box<Option<String>>,
}
