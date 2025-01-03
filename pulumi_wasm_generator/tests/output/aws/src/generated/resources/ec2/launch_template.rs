/// Provides an EC2 launch template resource. Can be used to create instances or auto scaling groups.
///
/// ## Import
///
/// Using `pulumi import`, import Launch Templates using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/launchTemplate:LaunchTemplate web lt-12345678
/// ```
pub mod launch_template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LaunchTemplateArgs {
        /// Specify volumes to attach to the instance besides the volumes specified by the AMI.
        /// See Block Devices below for details.
        #[builder(into, default)]
        pub block_device_mappings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ec2::LaunchTemplateBlockDeviceMapping>>,
        >,
        /// Targeting for EC2 capacity reservations. See Capacity Reservation Specification below for more details.
        #[builder(into, default)]
        pub capacity_reservation_specification: pulumi_wasm_rust::Output<
            Option<
                super::super::types::ec2::LaunchTemplateCapacityReservationSpecification,
            >,
        >,
        /// The CPU options for the instance. See CPU Options below for more details.
        #[builder(into, default)]
        pub cpu_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateCpuOptions>,
        >,
        /// Customize the credit specification of the instance. See Credit
        /// Specification below for more details.
        #[builder(into, default)]
        pub credit_specification: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateCreditSpecification>,
        >,
        /// Default Version of the launch template.
        #[builder(into, default)]
        pub default_version: pulumi_wasm_rust::Output<Option<i32>>,
        /// Description of the launch template.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// If true, enables [EC2 Instance Stop Protection](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-stop-protection.html).
        #[builder(into, default)]
        pub disable_api_stop: pulumi_wasm_rust::Output<Option<bool>>,
        /// If `true`, enables [EC2 Instance
        /// Termination Protection](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_ChangingDisableAPITermination.html)
        #[builder(into, default)]
        pub disable_api_termination: pulumi_wasm_rust::Output<Option<bool>>,
        /// If `true`, the launched EC2 instance will be EBS-optimized.
        #[builder(into, default)]
        pub ebs_optimized: pulumi_wasm_rust::Output<Option<String>>,
        /// The elastic GPU to attach to the instance. See Elastic GPU
        /// below for more details.
        #[builder(into, default)]
        pub elastic_gpu_specifications: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ec2::LaunchTemplateElasticGpuSpecification>>,
        >,
        /// Configuration block containing an Elastic Inference Accelerator to attach to the instance. See Elastic Inference Accelerator below for more details.
        #[builder(into, default)]
        pub elastic_inference_accelerator: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateElasticInferenceAccelerator>,
        >,
        /// Enable Nitro Enclaves on launched instances. See Enclave Options below for more details.
        #[builder(into, default)]
        pub enclave_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateEnclaveOptions>,
        >,
        /// The hibernation options for the instance. See Hibernation Options below for more details.
        #[builder(into, default)]
        pub hibernation_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateHibernationOptions>,
        >,
        /// The IAM Instance Profile to launch the instance with. See Instance Profile
        /// below for more details.
        #[builder(into, default)]
        pub iam_instance_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateIamInstanceProfile>,
        >,
        /// The AMI from which to launch the instance or use a Systems Manager parameter convention e.g. `resolve:ssm:parameter-name`. See [docs](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/create-launch-template.html#use-an-ssm-parameter-instead-of-an-ami-id) for more details.
        #[builder(into, default)]
        pub image_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Shutdown behavior for the instance. Can be `stop` or `terminate`.
        /// (Default: `stop`).
        #[builder(into, default)]
        pub instance_initiated_shutdown_behavior: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The market (purchasing) option for the instance. See Market Options
        /// below for details.
        #[builder(into, default)]
        pub instance_market_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateInstanceMarketOptions>,
        >,
        /// The attribute requirements for the type of instance. If present then `instance_type` cannot be present.
        #[builder(into, default)]
        pub instance_requirements: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateInstanceRequirements>,
        >,
        /// The type of the instance. If present then `instance_requirements` cannot be present.
        #[builder(into, default)]
        pub instance_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The kernel ID.
        #[builder(into, default)]
        pub kernel_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The key name to use for the instance.
        #[builder(into, default)]
        pub key_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of license specifications to associate with. See License Specification below for more details.
        #[builder(into, default)]
        pub license_specifications: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ec2::LaunchTemplateLicenseSpecification>>,
        >,
        /// The maintenance options for the instance. See Maintenance Options below for more details.
        #[builder(into, default)]
        pub maintenance_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateMaintenanceOptions>,
        >,
        /// Customize the metadata options for the instance. See Metadata Options below for more details.
        #[builder(into, default)]
        pub metadata_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateMetadataOptions>,
        >,
        /// The monitoring option for the instance. See Monitoring below for more details.
        #[builder(into, default)]
        pub monitoring: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateMonitoring>,
        >,
        /// The name of the launch template. If you leave this blank, the provider will auto-generate a unique name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Customize network interfaces to be attached at instance boot time. See Network
        /// Interfaces below for more details.
        #[builder(into, default)]
        pub network_interfaces: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ec2::LaunchTemplateNetworkInterface>>,
        >,
        /// The placement of the instance. See Placement below for more details.
        #[builder(into, default)]
        pub placement: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplatePlacement>,
        >,
        /// The options for the instance hostname. The default values are inherited from the subnet. See Private DNS Name Options below for more details.
        #[builder(into, default)]
        pub private_dns_name_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplatePrivateDnsNameOptions>,
        >,
        /// The ID of the RAM disk.
        #[builder(into, default)]
        pub ram_disk_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of security group names to associate with. If you are creating Instances in a VPC, use
        /// `vpc_security_group_ids` instead.
        #[builder(into, default)]
        pub security_group_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The tags to apply to the resources during launch. See Tag Specifications below for more details. Default tags are currently not propagated to ASG created resources so you may wish to inject your default tags into this variable against the relevant child resource types created.
        #[builder(into, default)]
        pub tag_specifications: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ec2::LaunchTemplateTagSpecification>>,
        >,
        /// A map of tags to assign to the launch template. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether to update Default Version each update. Conflicts with `default_version`.
        #[builder(into, default)]
        pub update_default_version: pulumi_wasm_rust::Output<Option<bool>>,
        /// The base64-encoded user data to provide when launching the instance.
        #[builder(into, default)]
        pub user_data: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of security group IDs to associate with. Conflicts with `network_interfaces.security_groups`
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct LaunchTemplateResult {
        /// Amazon Resource Name (ARN) of the launch template.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Specify volumes to attach to the instance besides the volumes specified by the AMI.
        /// See Block Devices below for details.
        pub block_device_mappings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ec2::LaunchTemplateBlockDeviceMapping>>,
        >,
        /// Targeting for EC2 capacity reservations. See Capacity Reservation Specification below for more details.
        pub capacity_reservation_specification: pulumi_wasm_rust::Output<
            Option<
                super::super::types::ec2::LaunchTemplateCapacityReservationSpecification,
            >,
        >,
        /// The CPU options for the instance. See CPU Options below for more details.
        pub cpu_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateCpuOptions>,
        >,
        /// Customize the credit specification of the instance. See Credit
        /// Specification below for more details.
        pub credit_specification: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateCreditSpecification>,
        >,
        /// Default Version of the launch template.
        pub default_version: pulumi_wasm_rust::Output<i32>,
        /// Description of the launch template.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// If true, enables [EC2 Instance Stop Protection](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-stop-protection.html).
        pub disable_api_stop: pulumi_wasm_rust::Output<Option<bool>>,
        /// If `true`, enables [EC2 Instance
        /// Termination Protection](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_ChangingDisableAPITermination.html)
        pub disable_api_termination: pulumi_wasm_rust::Output<Option<bool>>,
        /// If `true`, the launched EC2 instance will be EBS-optimized.
        pub ebs_optimized: pulumi_wasm_rust::Output<Option<String>>,
        /// The elastic GPU to attach to the instance. See Elastic GPU
        /// below for more details.
        pub elastic_gpu_specifications: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ec2::LaunchTemplateElasticGpuSpecification>>,
        >,
        /// Configuration block containing an Elastic Inference Accelerator to attach to the instance. See Elastic Inference Accelerator below for more details.
        pub elastic_inference_accelerator: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateElasticInferenceAccelerator>,
        >,
        /// Enable Nitro Enclaves on launched instances. See Enclave Options below for more details.
        pub enclave_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateEnclaveOptions>,
        >,
        /// The hibernation options for the instance. See Hibernation Options below for more details.
        pub hibernation_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateHibernationOptions>,
        >,
        /// The IAM Instance Profile to launch the instance with. See Instance Profile
        /// below for more details.
        pub iam_instance_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateIamInstanceProfile>,
        >,
        /// The AMI from which to launch the instance or use a Systems Manager parameter convention e.g. `resolve:ssm:parameter-name`. See [docs](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/create-launch-template.html#use-an-ssm-parameter-instead-of-an-ami-id) for more details.
        pub image_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Shutdown behavior for the instance. Can be `stop` or `terminate`.
        /// (Default: `stop`).
        pub instance_initiated_shutdown_behavior: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The market (purchasing) option for the instance. See Market Options
        /// below for details.
        pub instance_market_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateInstanceMarketOptions>,
        >,
        /// The attribute requirements for the type of instance. If present then `instance_type` cannot be present.
        pub instance_requirements: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateInstanceRequirements>,
        >,
        /// The type of the instance. If present then `instance_requirements` cannot be present.
        pub instance_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The kernel ID.
        pub kernel_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The key name to use for the instance.
        pub key_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The latest version of the launch template.
        pub latest_version: pulumi_wasm_rust::Output<i32>,
        /// A list of license specifications to associate with. See License Specification below for more details.
        pub license_specifications: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ec2::LaunchTemplateLicenseSpecification>>,
        >,
        /// The maintenance options for the instance. See Maintenance Options below for more details.
        pub maintenance_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateMaintenanceOptions>,
        >,
        /// Customize the metadata options for the instance. See Metadata Options below for more details.
        pub metadata_options: pulumi_wasm_rust::Output<
            super::super::types::ec2::LaunchTemplateMetadataOptions,
        >,
        /// The monitoring option for the instance. See Monitoring below for more details.
        pub monitoring: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplateMonitoring>,
        >,
        /// The name of the launch template. If you leave this blank, the provider will auto-generate a unique name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// Customize network interfaces to be attached at instance boot time. See Network
        /// Interfaces below for more details.
        pub network_interfaces: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ec2::LaunchTemplateNetworkInterface>>,
        >,
        /// The placement of the instance. See Placement below for more details.
        pub placement: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplatePlacement>,
        >,
        /// The options for the instance hostname. The default values are inherited from the subnet. See Private DNS Name Options below for more details.
        pub private_dns_name_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchTemplatePrivateDnsNameOptions>,
        >,
        /// The ID of the RAM disk.
        pub ram_disk_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of security group names to associate with. If you are creating Instances in a VPC, use
        /// `vpc_security_group_ids` instead.
        pub security_group_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The tags to apply to the resources during launch. See Tag Specifications below for more details. Default tags are currently not propagated to ASG created resources so you may wish to inject your default tags into this variable against the relevant child resource types created.
        pub tag_specifications: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ec2::LaunchTemplateTagSpecification>>,
        >,
        /// A map of tags to assign to the launch template. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether to update Default Version each update. Conflicts with `default_version`.
        pub update_default_version: pulumi_wasm_rust::Output<Option<bool>>,
        /// The base64-encoded user data to provide when launching the instance.
        pub user_data: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of security group IDs to associate with. Conflicts with `network_interfaces.security_groups`
        pub vpc_security_group_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LaunchTemplateArgs) -> LaunchTemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let block_device_mappings_binding = args.block_device_mappings.get_inner();
        let capacity_reservation_specification_binding = args
            .capacity_reservation_specification
            .get_inner();
        let cpu_options_binding = args.cpu_options.get_inner();
        let credit_specification_binding = args.credit_specification.get_inner();
        let default_version_binding = args.default_version.get_inner();
        let description_binding = args.description.get_inner();
        let disable_api_stop_binding = args.disable_api_stop.get_inner();
        let disable_api_termination_binding = args.disable_api_termination.get_inner();
        let ebs_optimized_binding = args.ebs_optimized.get_inner();
        let elastic_gpu_specifications_binding = args
            .elastic_gpu_specifications
            .get_inner();
        let elastic_inference_accelerator_binding = args
            .elastic_inference_accelerator
            .get_inner();
        let enclave_options_binding = args.enclave_options.get_inner();
        let hibernation_options_binding = args.hibernation_options.get_inner();
        let iam_instance_profile_binding = args.iam_instance_profile.get_inner();
        let image_id_binding = args.image_id.get_inner();
        let instance_initiated_shutdown_behavior_binding = args
            .instance_initiated_shutdown_behavior
            .get_inner();
        let instance_market_options_binding = args.instance_market_options.get_inner();
        let instance_requirements_binding = args.instance_requirements.get_inner();
        let instance_type_binding = args.instance_type.get_inner();
        let kernel_id_binding = args.kernel_id.get_inner();
        let key_name_binding = args.key_name.get_inner();
        let license_specifications_binding = args.license_specifications.get_inner();
        let maintenance_options_binding = args.maintenance_options.get_inner();
        let metadata_options_binding = args.metadata_options.get_inner();
        let monitoring_binding = args.monitoring.get_inner();
        let name_binding = args.name.get_inner();
        let name_prefix_binding = args.name_prefix.get_inner();
        let network_interfaces_binding = args.network_interfaces.get_inner();
        let placement_binding = args.placement.get_inner();
        let private_dns_name_options_binding = args.private_dns_name_options.get_inner();
        let ram_disk_id_binding = args.ram_disk_id.get_inner();
        let security_group_names_binding = args.security_group_names.get_inner();
        let tag_specifications_binding = args.tag_specifications.get_inner();
        let tags_binding = args.tags.get_inner();
        let update_default_version_binding = args.update_default_version.get_inner();
        let user_data_binding = args.user_data.get_inner();
        let vpc_security_group_ids_binding = args.vpc_security_group_ids.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/launchTemplate:LaunchTemplate".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "blockDeviceMappings".into(),
                    value: &block_device_mappings_binding,
                },
                register_interface::ObjectField {
                    name: "capacityReservationSpecification".into(),
                    value: &capacity_reservation_specification_binding,
                },
                register_interface::ObjectField {
                    name: "cpuOptions".into(),
                    value: &cpu_options_binding,
                },
                register_interface::ObjectField {
                    name: "creditSpecification".into(),
                    value: &credit_specification_binding,
                },
                register_interface::ObjectField {
                    name: "defaultVersion".into(),
                    value: &default_version_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "disableApiStop".into(),
                    value: &disable_api_stop_binding,
                },
                register_interface::ObjectField {
                    name: "disableApiTermination".into(),
                    value: &disable_api_termination_binding,
                },
                register_interface::ObjectField {
                    name: "ebsOptimized".into(),
                    value: &ebs_optimized_binding,
                },
                register_interface::ObjectField {
                    name: "elasticGpuSpecifications".into(),
                    value: &elastic_gpu_specifications_binding,
                },
                register_interface::ObjectField {
                    name: "elasticInferenceAccelerator".into(),
                    value: &elastic_inference_accelerator_binding,
                },
                register_interface::ObjectField {
                    name: "enclaveOptions".into(),
                    value: &enclave_options_binding,
                },
                register_interface::ObjectField {
                    name: "hibernationOptions".into(),
                    value: &hibernation_options_binding,
                },
                register_interface::ObjectField {
                    name: "iamInstanceProfile".into(),
                    value: &iam_instance_profile_binding,
                },
                register_interface::ObjectField {
                    name: "imageId".into(),
                    value: &image_id_binding,
                },
                register_interface::ObjectField {
                    name: "instanceInitiatedShutdownBehavior".into(),
                    value: &instance_initiated_shutdown_behavior_binding,
                },
                register_interface::ObjectField {
                    name: "instanceMarketOptions".into(),
                    value: &instance_market_options_binding,
                },
                register_interface::ObjectField {
                    name: "instanceRequirements".into(),
                    value: &instance_requirements_binding,
                },
                register_interface::ObjectField {
                    name: "instanceType".into(),
                    value: &instance_type_binding,
                },
                register_interface::ObjectField {
                    name: "kernelId".into(),
                    value: &kernel_id_binding,
                },
                register_interface::ObjectField {
                    name: "keyName".into(),
                    value: &key_name_binding,
                },
                register_interface::ObjectField {
                    name: "licenseSpecifications".into(),
                    value: &license_specifications_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceOptions".into(),
                    value: &maintenance_options_binding,
                },
                register_interface::ObjectField {
                    name: "metadataOptions".into(),
                    value: &metadata_options_binding,
                },
                register_interface::ObjectField {
                    name: "monitoring".into(),
                    value: &monitoring_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "networkInterfaces".into(),
                    value: &network_interfaces_binding,
                },
                register_interface::ObjectField {
                    name: "placement".into(),
                    value: &placement_binding,
                },
                register_interface::ObjectField {
                    name: "privateDnsNameOptions".into(),
                    value: &private_dns_name_options_binding,
                },
                register_interface::ObjectField {
                    name: "ramDiskId".into(),
                    value: &ram_disk_id_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroupNames".into(),
                    value: &security_group_names_binding,
                },
                register_interface::ObjectField {
                    name: "tagSpecifications".into(),
                    value: &tag_specifications_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "updateDefaultVersion".into(),
                    value: &update_default_version_binding,
                },
                register_interface::ObjectField {
                    name: "userData".into(),
                    value: &user_data_binding,
                },
                register_interface::ObjectField {
                    name: "vpcSecurityGroupIds".into(),
                    value: &vpc_security_group_ids_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "blockDeviceMappings".into(),
                },
                register_interface::ResultField {
                    name: "capacityReservationSpecification".into(),
                },
                register_interface::ResultField {
                    name: "cpuOptions".into(),
                },
                register_interface::ResultField {
                    name: "creditSpecification".into(),
                },
                register_interface::ResultField {
                    name: "defaultVersion".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "disableApiStop".into(),
                },
                register_interface::ResultField {
                    name: "disableApiTermination".into(),
                },
                register_interface::ResultField {
                    name: "ebsOptimized".into(),
                },
                register_interface::ResultField {
                    name: "elasticGpuSpecifications".into(),
                },
                register_interface::ResultField {
                    name: "elasticInferenceAccelerator".into(),
                },
                register_interface::ResultField {
                    name: "enclaveOptions".into(),
                },
                register_interface::ResultField {
                    name: "hibernationOptions".into(),
                },
                register_interface::ResultField {
                    name: "iamInstanceProfile".into(),
                },
                register_interface::ResultField {
                    name: "imageId".into(),
                },
                register_interface::ResultField {
                    name: "instanceInitiatedShutdownBehavior".into(),
                },
                register_interface::ResultField {
                    name: "instanceMarketOptions".into(),
                },
                register_interface::ResultField {
                    name: "instanceRequirements".into(),
                },
                register_interface::ResultField {
                    name: "instanceType".into(),
                },
                register_interface::ResultField {
                    name: "kernelId".into(),
                },
                register_interface::ResultField {
                    name: "keyName".into(),
                },
                register_interface::ResultField {
                    name: "latestVersion".into(),
                },
                register_interface::ResultField {
                    name: "licenseSpecifications".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceOptions".into(),
                },
                register_interface::ResultField {
                    name: "metadataOptions".into(),
                },
                register_interface::ResultField {
                    name: "monitoring".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaces".into(),
                },
                register_interface::ResultField {
                    name: "placement".into(),
                },
                register_interface::ResultField {
                    name: "privateDnsNameOptions".into(),
                },
                register_interface::ResultField {
                    name: "ramDiskId".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupNames".into(),
                },
                register_interface::ResultField {
                    name: "tagSpecifications".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "updateDefaultVersion".into(),
                },
                register_interface::ResultField {
                    name: "userData".into(),
                },
                register_interface::ResultField {
                    name: "vpcSecurityGroupIds".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LaunchTemplateResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            block_device_mappings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blockDeviceMappings").unwrap(),
            ),
            capacity_reservation_specification: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacityReservationSpecification").unwrap(),
            ),
            cpu_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cpuOptions").unwrap(),
            ),
            credit_specification: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creditSpecification").unwrap(),
            ),
            default_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultVersion").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disable_api_stop: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableApiStop").unwrap(),
            ),
            disable_api_termination: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableApiTermination").unwrap(),
            ),
            ebs_optimized: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsOptimized").unwrap(),
            ),
            elastic_gpu_specifications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("elasticGpuSpecifications").unwrap(),
            ),
            elastic_inference_accelerator: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("elasticInferenceAccelerator").unwrap(),
            ),
            enclave_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enclaveOptions").unwrap(),
            ),
            hibernation_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hibernationOptions").unwrap(),
            ),
            iam_instance_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamInstanceProfile").unwrap(),
            ),
            image_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageId").unwrap(),
            ),
            instance_initiated_shutdown_behavior: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceInitiatedShutdownBehavior").unwrap(),
            ),
            instance_market_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceMarketOptions").unwrap(),
            ),
            instance_requirements: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceRequirements").unwrap(),
            ),
            instance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceType").unwrap(),
            ),
            kernel_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kernelId").unwrap(),
            ),
            key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyName").unwrap(),
            ),
            latest_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latestVersion").unwrap(),
            ),
            license_specifications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseSpecifications").unwrap(),
            ),
            maintenance_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceOptions").unwrap(),
            ),
            metadata_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadataOptions").unwrap(),
            ),
            monitoring: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monitoring").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
            ),
            network_interfaces: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaces").unwrap(),
            ),
            placement: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("placement").unwrap(),
            ),
            private_dns_name_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateDnsNameOptions").unwrap(),
            ),
            ram_disk_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ramDiskId").unwrap(),
            ),
            security_group_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupNames").unwrap(),
            ),
            tag_specifications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagSpecifications").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            update_default_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateDefaultVersion").unwrap(),
            ),
            user_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userData").unwrap(),
            ),
            vpc_security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcSecurityGroupIds").unwrap(),
            ),
        }
    }
}
