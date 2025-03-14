/// Provides an EC2 launch template resource. Can be used to create instances or auto scaling groups.
///
/// ## Import
///
/// Using `pulumi import`, import Launch Templates using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/launchTemplate:LaunchTemplate web lt-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod launch_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LaunchTemplateArgs {
        /// Specify volumes to attach to the instance besides the volumes specified by the AMI.
        /// See Block Devices below for details.
        #[builder(into, default)]
        pub block_device_mappings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::LaunchTemplateBlockDeviceMapping>>,
        >,
        /// Targeting for EC2 capacity reservations. See Capacity Reservation Specification below for more details.
        #[builder(into, default)]
        pub capacity_reservation_specification: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::ec2::LaunchTemplateCapacityReservationSpecification,
            >,
        >,
        /// The CPU options for the instance. See CPU Options below for more details.
        #[builder(into, default)]
        pub cpu_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::LaunchTemplateCpuOptions>,
        >,
        /// Customize the credit specification of the instance. See Credit
        /// Specification below for more details.
        #[builder(into, default)]
        pub credit_specification: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::LaunchTemplateCreditSpecification>,
        >,
        /// Default Version of the launch template.
        #[builder(into, default)]
        pub default_version: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Description of the launch template.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If true, enables [EC2 Instance Stop Protection](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-stop-protection.html).
        #[builder(into, default)]
        pub disable_api_stop: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If `true`, enables [EC2 Instance
        /// Termination Protection](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_ChangingDisableAPITermination.html)
        #[builder(into, default)]
        pub disable_api_termination: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If `true`, the launched EC2 instance will be EBS-optimized.
        #[builder(into, default)]
        pub ebs_optimized: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The elastic GPU to attach to the instance. See Elastic GPU
        /// below for more details.
        #[builder(into, default)]
        pub elastic_gpu_specifications: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::LaunchTemplateElasticGpuSpecification>>,
        >,
        /// Configuration block containing an Elastic Inference Accelerator to attach to the instance. See Elastic Inference Accelerator below for more details.
        #[builder(into, default)]
        pub elastic_inference_accelerator: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::LaunchTemplateElasticInferenceAccelerator>,
        >,
        /// Enable Nitro Enclaves on launched instances. See Enclave Options below for more details.
        #[builder(into, default)]
        pub enclave_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::LaunchTemplateEnclaveOptions>,
        >,
        /// The hibernation options for the instance. See Hibernation Options below for more details.
        #[builder(into, default)]
        pub hibernation_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::LaunchTemplateHibernationOptions>,
        >,
        /// The IAM Instance Profile to launch the instance with. See Instance Profile
        /// below for more details.
        #[builder(into, default)]
        pub iam_instance_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::LaunchTemplateIamInstanceProfile>,
        >,
        /// The AMI from which to launch the instance or use a Systems Manager parameter convention e.g. `resolve:ssm:parameter-name`. See [docs](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/create-launch-template.html#use-an-ssm-parameter-instead-of-an-ami-id) for more details.
        #[builder(into, default)]
        pub image_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Shutdown behavior for the instance. Can be `stop` or `terminate`.
        /// (Default: `stop`).
        #[builder(into, default)]
        pub instance_initiated_shutdown_behavior: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The market (purchasing) option for the instance. See Market Options
        /// below for details.
        #[builder(into, default)]
        pub instance_market_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::LaunchTemplateInstanceMarketOptions>,
        >,
        /// The attribute requirements for the type of instance. If present then `instance_type` cannot be present.
        #[builder(into, default)]
        pub instance_requirements: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::LaunchTemplateInstanceRequirements>,
        >,
        /// The type of the instance. If present then `instance_requirements` cannot be present.
        #[builder(into, default)]
        pub instance_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The kernel ID.
        #[builder(into, default)]
        pub kernel_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The key name to use for the instance.
        #[builder(into, default)]
        pub key_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of license specifications to associate with. See License Specification below for more details.
        #[builder(into, default)]
        pub license_specifications: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::LaunchTemplateLicenseSpecification>>,
        >,
        /// The maintenance options for the instance. See Maintenance Options below for more details.
        #[builder(into, default)]
        pub maintenance_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::LaunchTemplateMaintenanceOptions>,
        >,
        /// Customize the metadata options for the instance. See Metadata Options below for more details.
        #[builder(into, default)]
        pub metadata_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::LaunchTemplateMetadataOptions>,
        >,
        /// The monitoring option for the instance. See Monitoring below for more details.
        #[builder(into, default)]
        pub monitoring: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::LaunchTemplateMonitoring>,
        >,
        /// The name of the launch template. If you leave this blank, the provider will auto-generate a unique name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Customize network interfaces to be attached at instance boot time. See Network
        /// Interfaces below for more details.
        #[builder(into, default)]
        pub network_interfaces: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::LaunchTemplateNetworkInterface>>,
        >,
        /// The placement of the instance. See Placement below for more details.
        #[builder(into, default)]
        pub placement: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::LaunchTemplatePlacement>,
        >,
        /// The options for the instance hostname. The default values are inherited from the subnet. See Private DNS Name Options below for more details.
        #[builder(into, default)]
        pub private_dns_name_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::LaunchTemplatePrivateDnsNameOptions>,
        >,
        /// The ID of the RAM disk.
        #[builder(into, default)]
        pub ram_disk_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of security group names to associate with. If you are creating Instances in a VPC, use
        /// `vpc_security_group_ids` instead.
        #[builder(into, default)]
        pub security_group_names: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The tags to apply to the resources during launch. See Tag Specifications below for more details. Default tags are currently not propagated to ASG created resources so you may wish to inject your default tags into this variable against the relevant child resource types created.
        #[builder(into, default)]
        pub tag_specifications: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::LaunchTemplateTagSpecification>>,
        >,
        /// A map of tags to assign to the launch template. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether to update Default Version each update. Conflicts with `default_version`.
        #[builder(into, default)]
        pub update_default_version: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The base64-encoded user data to provide when launching the instance.
        #[builder(into, default)]
        pub user_data: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of security group IDs to associate with. Conflicts with `network_interfaces.security_groups`
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LaunchTemplateResult {
        /// Amazon Resource Name (ARN) of the launch template.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Specify volumes to attach to the instance besides the volumes specified by the AMI.
        /// See Block Devices below for details.
        pub block_device_mappings: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ec2::LaunchTemplateBlockDeviceMapping>>,
        >,
        /// Targeting for EC2 capacity reservations. See Capacity Reservation Specification below for more details.
        pub capacity_reservation_specification: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::ec2::LaunchTemplateCapacityReservationSpecification,
            >,
        >,
        /// The CPU options for the instance. See CPU Options below for more details.
        pub cpu_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateCpuOptions>,
        >,
        /// Customize the credit specification of the instance. See Credit
        /// Specification below for more details.
        pub credit_specification: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateCreditSpecification>,
        >,
        /// Default Version of the launch template.
        pub default_version: pulumi_gestalt_rust::Output<i32>,
        /// Description of the launch template.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// If true, enables [EC2 Instance Stop Protection](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-stop-protection.html).
        pub disable_api_stop: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If `true`, enables [EC2 Instance
        /// Termination Protection](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_ChangingDisableAPITermination.html)
        pub disable_api_termination: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If `true`, the launched EC2 instance will be EBS-optimized.
        pub ebs_optimized: pulumi_gestalt_rust::Output<Option<String>>,
        /// The elastic GPU to attach to the instance. See Elastic GPU
        /// below for more details.
        pub elastic_gpu_specifications: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ec2::LaunchTemplateElasticGpuSpecification>>,
        >,
        /// Configuration block containing an Elastic Inference Accelerator to attach to the instance. See Elastic Inference Accelerator below for more details.
        pub elastic_inference_accelerator: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateElasticInferenceAccelerator>,
        >,
        /// Enable Nitro Enclaves on launched instances. See Enclave Options below for more details.
        pub enclave_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateEnclaveOptions>,
        >,
        /// The hibernation options for the instance. See Hibernation Options below for more details.
        pub hibernation_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateHibernationOptions>,
        >,
        /// The IAM Instance Profile to launch the instance with. See Instance Profile
        /// below for more details.
        pub iam_instance_profile: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateIamInstanceProfile>,
        >,
        /// The AMI from which to launch the instance or use a Systems Manager parameter convention e.g. `resolve:ssm:parameter-name`. See [docs](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/create-launch-template.html#use-an-ssm-parameter-instead-of-an-ami-id) for more details.
        pub image_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Shutdown behavior for the instance. Can be `stop` or `terminate`.
        /// (Default: `stop`).
        pub instance_initiated_shutdown_behavior: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The market (purchasing) option for the instance. See Market Options
        /// below for details.
        pub instance_market_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateInstanceMarketOptions>,
        >,
        /// The attribute requirements for the type of instance. If present then `instance_type` cannot be present.
        pub instance_requirements: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateInstanceRequirements>,
        >,
        /// The type of the instance. If present then `instance_requirements` cannot be present.
        pub instance_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The kernel ID.
        pub kernel_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The key name to use for the instance.
        pub key_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The latest version of the launch template.
        pub latest_version: pulumi_gestalt_rust::Output<i32>,
        /// A list of license specifications to associate with. See License Specification below for more details.
        pub license_specifications: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ec2::LaunchTemplateLicenseSpecification>>,
        >,
        /// The maintenance options for the instance. See Maintenance Options below for more details.
        pub maintenance_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateMaintenanceOptions>,
        >,
        /// Customize the metadata options for the instance. See Metadata Options below for more details.
        pub metadata_options: pulumi_gestalt_rust::Output<
            super::super::types::ec2::LaunchTemplateMetadataOptions,
        >,
        /// The monitoring option for the instance. See Monitoring below for more details.
        pub monitoring: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateMonitoring>,
        >,
        /// The name of the launch template. If you leave this blank, the provider will auto-generate a unique name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// Customize network interfaces to be attached at instance boot time. See Network
        /// Interfaces below for more details.
        pub network_interfaces: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ec2::LaunchTemplateNetworkInterface>>,
        >,
        /// The placement of the instance. See Placement below for more details.
        pub placement: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::LaunchTemplatePlacement>,
        >,
        /// The options for the instance hostname. The default values are inherited from the subnet. See Private DNS Name Options below for more details.
        pub private_dns_name_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::LaunchTemplatePrivateDnsNameOptions>,
        >,
        /// The ID of the RAM disk.
        pub ram_disk_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of security group names to associate with. If you are creating Instances in a VPC, use
        /// `vpc_security_group_ids` instead.
        pub security_group_names: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The tags to apply to the resources during launch. See Tag Specifications below for more details. Default tags are currently not propagated to ASG created resources so you may wish to inject your default tags into this variable against the relevant child resource types created.
        pub tag_specifications: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ec2::LaunchTemplateTagSpecification>>,
        >,
        /// A map of tags to assign to the launch template. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether to update Default Version each update. Conflicts with `default_version`.
        pub update_default_version: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The base64-encoded user data to provide when launching the instance.
        pub user_data: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of security group IDs to associate with. Conflicts with `network_interfaces.security_groups`
        pub vpc_security_group_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LaunchTemplateArgs,
    ) -> LaunchTemplateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let block_device_mappings_binding = args
            .block_device_mappings
            .get_output(context);
        let capacity_reservation_specification_binding = args
            .capacity_reservation_specification
            .get_output(context);
        let cpu_options_binding = args.cpu_options.get_output(context);
        let credit_specification_binding = args.credit_specification.get_output(context);
        let default_version_binding = args.default_version.get_output(context);
        let description_binding = args.description.get_output(context);
        let disable_api_stop_binding = args.disable_api_stop.get_output(context);
        let disable_api_termination_binding = args
            .disable_api_termination
            .get_output(context);
        let ebs_optimized_binding = args.ebs_optimized.get_output(context);
        let elastic_gpu_specifications_binding = args
            .elastic_gpu_specifications
            .get_output(context);
        let elastic_inference_accelerator_binding = args
            .elastic_inference_accelerator
            .get_output(context);
        let enclave_options_binding = args.enclave_options.get_output(context);
        let hibernation_options_binding = args.hibernation_options.get_output(context);
        let iam_instance_profile_binding = args.iam_instance_profile.get_output(context);
        let image_id_binding = args.image_id.get_output(context);
        let instance_initiated_shutdown_behavior_binding = args
            .instance_initiated_shutdown_behavior
            .get_output(context);
        let instance_market_options_binding = args
            .instance_market_options
            .get_output(context);
        let instance_requirements_binding = args
            .instance_requirements
            .get_output(context);
        let instance_type_binding = args.instance_type.get_output(context);
        let kernel_id_binding = args.kernel_id.get_output(context);
        let key_name_binding = args.key_name.get_output(context);
        let license_specifications_binding = args
            .license_specifications
            .get_output(context);
        let maintenance_options_binding = args.maintenance_options.get_output(context);
        let metadata_options_binding = args.metadata_options.get_output(context);
        let monitoring_binding = args.monitoring.get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let network_interfaces_binding = args.network_interfaces.get_output(context);
        let placement_binding = args.placement.get_output(context);
        let private_dns_name_options_binding = args
            .private_dns_name_options
            .get_output(context);
        let ram_disk_id_binding = args.ram_disk_id.get_output(context);
        let security_group_names_binding = args.security_group_names.get_output(context);
        let tag_specifications_binding = args.tag_specifications.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let update_default_version_binding = args
            .update_default_version
            .get_output(context);
        let user_data_binding = args.user_data.get_output(context);
        let vpc_security_group_ids_binding = args
            .vpc_security_group_ids
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/launchTemplate:LaunchTemplate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blockDeviceMappings".into(),
                    value: &block_device_mappings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacityReservationSpecification".into(),
                    value: &capacity_reservation_specification_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cpuOptions".into(),
                    value: &cpu_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "creditSpecification".into(),
                    value: &credit_specification_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultVersion".into(),
                    value: &default_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disableApiStop".into(),
                    value: &disable_api_stop_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disableApiTermination".into(),
                    value: &disable_api_termination_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ebsOptimized".into(),
                    value: &ebs_optimized_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "elasticGpuSpecifications".into(),
                    value: &elastic_gpu_specifications_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "elasticInferenceAccelerator".into(),
                    value: &elastic_inference_accelerator_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enclaveOptions".into(),
                    value: &enclave_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hibernationOptions".into(),
                    value: &hibernation_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iamInstanceProfile".into(),
                    value: &iam_instance_profile_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageId".into(),
                    value: &image_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceInitiatedShutdownBehavior".into(),
                    value: &instance_initiated_shutdown_behavior_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceMarketOptions".into(),
                    value: &instance_market_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceRequirements".into(),
                    value: &instance_requirements_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceType".into(),
                    value: &instance_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kernelId".into(),
                    value: &kernel_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyName".into(),
                    value: &key_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "licenseSpecifications".into(),
                    value: &license_specifications_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maintenanceOptions".into(),
                    value: &maintenance_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadataOptions".into(),
                    value: &metadata_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "monitoring".into(),
                    value: &monitoring_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkInterfaces".into(),
                    value: &network_interfaces_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "placement".into(),
                    value: &placement_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateDnsNameOptions".into(),
                    value: &private_dns_name_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ramDiskId".into(),
                    value: &ram_disk_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroupNames".into(),
                    value: &security_group_names_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tagSpecifications".into(),
                    value: &tag_specifications_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "updateDefaultVersion".into(),
                    value: &update_default_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userData".into(),
                    value: &user_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcSecurityGroupIds".into(),
                    value: &vpc_security_group_ids_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LaunchTemplateResult {
            arn: o.get_field("arn"),
            block_device_mappings: o.get_field("blockDeviceMappings"),
            capacity_reservation_specification: o
                .get_field("capacityReservationSpecification"),
            cpu_options: o.get_field("cpuOptions"),
            credit_specification: o.get_field("creditSpecification"),
            default_version: o.get_field("defaultVersion"),
            description: o.get_field("description"),
            disable_api_stop: o.get_field("disableApiStop"),
            disable_api_termination: o.get_field("disableApiTermination"),
            ebs_optimized: o.get_field("ebsOptimized"),
            elastic_gpu_specifications: o.get_field("elasticGpuSpecifications"),
            elastic_inference_accelerator: o.get_field("elasticInferenceAccelerator"),
            enclave_options: o.get_field("enclaveOptions"),
            hibernation_options: o.get_field("hibernationOptions"),
            iam_instance_profile: o.get_field("iamInstanceProfile"),
            image_id: o.get_field("imageId"),
            instance_initiated_shutdown_behavior: o
                .get_field("instanceInitiatedShutdownBehavior"),
            instance_market_options: o.get_field("instanceMarketOptions"),
            instance_requirements: o.get_field("instanceRequirements"),
            instance_type: o.get_field("instanceType"),
            kernel_id: o.get_field("kernelId"),
            key_name: o.get_field("keyName"),
            latest_version: o.get_field("latestVersion"),
            license_specifications: o.get_field("licenseSpecifications"),
            maintenance_options: o.get_field("maintenanceOptions"),
            metadata_options: o.get_field("metadataOptions"),
            monitoring: o.get_field("monitoring"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            network_interfaces: o.get_field("networkInterfaces"),
            placement: o.get_field("placement"),
            private_dns_name_options: o.get_field("privateDnsNameOptions"),
            ram_disk_id: o.get_field("ramDiskId"),
            security_group_names: o.get_field("securityGroupNames"),
            tag_specifications: o.get_field("tagSpecifications"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            update_default_version: o.get_field("updateDefaultVersion"),
            user_data: o.get_field("userData"),
            vpc_security_group_ids: o.get_field("vpcSecurityGroupIds"),
        }
    }
}
