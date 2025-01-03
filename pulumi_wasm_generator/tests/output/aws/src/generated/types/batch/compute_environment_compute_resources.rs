#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ComputeEnvironmentComputeResources {
    /// The allocation strategy to use for the compute resource in case not enough instances of the best fitting instance type can be allocated. For valid values, refer to the [AWS documentation](https://docs.aws.amazon.com/batch/latest/APIReference/API_ComputeResource.html#Batch-Type-ComputeResource-allocationStrategy). Defaults to `BEST_FIT`. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into, default)]
    #[serde(rename = "allocationStrategy")]
    pub r#allocation_strategy: Box<Option<String>>,
    /// Integer of maximum percentage that a Spot Instance price can be when compared with the On-Demand price for that instance type before instances are launched. For example, if your bid percentage is 20% (`20`), then the Spot price must be below 20% of the current On-Demand price for that EC2 instance. If you leave this field empty, the default value is 100% of the On-Demand price. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into, default)]
    #[serde(rename = "bidPercentage")]
    pub r#bid_percentage: Box<Option<i32>>,
    /// The desired number of EC2 vCPUS in the compute environment. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into, default)]
    #[serde(rename = "desiredVcpus")]
    pub r#desired_vcpus: Box<Option<i32>>,
    /// Provides information used to select Amazon Machine Images (AMIs) for EC2 instances in the compute environment. If Ec2Configuration isn't specified, the default is ECS_AL2. This parameter isn't applicable to jobs that are running on Fargate resources, and shouldn't be specified.
    #[builder(into, default)]
    #[serde(rename = "ec2Configurations")]
    pub r#ec_2_configurations: Box<Option<Vec<super::super::types::batch::ComputeEnvironmentComputeResourcesEc2Configuration>>>,
    /// The EC2 key pair that is used for instances launched in the compute environment. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into, default)]
    #[serde(rename = "ec2KeyPair")]
    pub r#ec_2_key_pair: Box<Option<String>>,
    /// The Amazon Machine Image (AMI) ID used for instances launched in the compute environment. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified. (Deprecated, use `ec2_configuration` `image_id_override` instead)
    #[builder(into, default)]
    #[serde(rename = "imageId")]
    pub r#image_id: Box<Option<String>>,
    /// The Amazon ECS instance role applied to Amazon EC2 instances in a compute environment. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into, default)]
    #[serde(rename = "instanceRole")]
    pub r#instance_role: Box<Option<String>>,
    /// A list of instance types that may be launched. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into, default)]
    #[serde(rename = "instanceTypes")]
    pub r#instance_types: Box<Option<Vec<String>>>,
    /// The launch template to use for your compute resources. See details below. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into, default)]
    #[serde(rename = "launchTemplate")]
    pub r#launch_template: Box<Option<super::super::types::batch::ComputeEnvironmentComputeResourcesLaunchTemplate>>,
    /// The maximum number of EC2 vCPUs that an environment can reach.
    #[builder(into)]
    #[serde(rename = "maxVcpus")]
    pub r#max_vcpus: Box<i32>,
    /// The minimum number of EC2 vCPUs that an environment should maintain. For `EC2` or `SPOT` compute environments, if the parameter is not explicitly defined, a `0` default value will be set. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into, default)]
    #[serde(rename = "minVcpus")]
    pub r#min_vcpus: Box<Option<i32>>,
    /// The Amazon EC2 placement group to associate with your compute resources.
    #[builder(into, default)]
    #[serde(rename = "placementGroup")]
    pub r#placement_group: Box<Option<String>>,
    /// A list of EC2 security group that are associated with instances launched in the compute environment. This parameter is required for Fargate compute environments.
    #[builder(into, default)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Box<Option<Vec<String>>>,
    /// The Amazon Resource Name (ARN) of the Amazon EC2 Spot Fleet IAM role applied to a SPOT compute environment. This parameter is required for SPOT compute environments. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into, default)]
    #[serde(rename = "spotIamFleetRole")]
    pub r#spot_iam_fleet_role: Box<Option<String>>,
    /// A list of VPC subnets into which the compute resources are launched.
    #[builder(into)]
    #[serde(rename = "subnets")]
    pub r#subnets: Box<Vec<String>>,
    /// Key-value pair tags to be applied to resources that are launched in the compute environment. This parameter isn't applicable to jobs running on Fargate resources, and shouldn't be specified.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<std::collections::HashMap<String, String>>>,
    /// The type of compute environment. Valid items are `EC2`, `SPOT`, `FARGATE` or `FARGATE_SPOT`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
