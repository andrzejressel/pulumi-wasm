/// Provides a resource to create a new launch configuration, used for autoscaling groups.
///
/// !> **WARNING:** The use of launch configurations is discouraged in favor of launch templates. Read more in the [AWS EC2 Documentation](https://docs.aws.amazon.com/autoscaling/ec2/userguide/launch-configurations.html).
///
/// > **Note** When using `aws.ec2.LaunchConfiguration` with `aws.autoscaling.Group`, it is recommended to use the `name_prefix` (Optional) instead of the `name` (Optional) attribute.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   asConf:
///     type: aws:ec2:LaunchConfiguration
///     name: as_conf
///     properties:
///       name: web_config
///       imageId: ${ubuntu.id}
///       instanceType: t2.micro
/// variables:
///   ubuntu:
///     fn::invoke:
///       function: aws:ec2:getAmi
///       arguments:
///         mostRecent: true
///         filters:
///           - name: name
///             values:
///               - ubuntu/images/hvm-ssd/ubuntu-trusty-14.04-amd64-server-*
///           - name: virtualization-type
///             values:
///               - hvm
///         owners:
///           - '099720109477'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import launch configurations using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/launchConfiguration:LaunchConfiguration as_conf pulumi-lg-123456
/// ```
pub mod launch_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LaunchConfigurationArgs {
        /// Associate a public ip address with an instance in a VPC.
        #[builder(into, default)]
        pub associate_public_ip_address: pulumi_wasm_rust::Output<Option<bool>>,
        /// Additional EBS block devices to attach to the instance. See Block Devices below for details.
        #[builder(into, default)]
        pub ebs_block_devices: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ec2::LaunchConfigurationEbsBlockDevice>>,
        >,
        /// If true, the launched EC2 instance will be EBS-optimized.
        #[builder(into, default)]
        pub ebs_optimized: pulumi_wasm_rust::Output<Option<bool>>,
        /// Enables/disables detailed monitoring. This is enabled by default.
        #[builder(into, default)]
        pub enable_monitoring: pulumi_wasm_rust::Output<Option<bool>>,
        /// Customize Ephemeral (also known as "Instance Store") volumes on the instance. See Block Devices below for details.
        #[builder(into, default)]
        pub ephemeral_block_devices: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::ec2::LaunchConfigurationEphemeralBlockDevice>,
            >,
        >,
        /// The name attribute of the IAM instance profile to associate with launched instances.
        #[builder(into, default)]
        pub iam_instance_profile: pulumi_wasm_rust::Output<Option<String>>,
        /// The EC2 image ID to launch.
        #[builder(into)]
        pub image_id: pulumi_wasm_rust::Output<String>,
        /// The size of instance to launch.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub instance_type: pulumi_wasm_rust::Output<String>,
        /// The key name that should be used for the instance.
        #[builder(into, default)]
        pub key_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The metadata options for the instance.
        #[builder(into, default)]
        pub metadata_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchConfigurationMetadataOptions>,
        >,
        /// The name of the launch configuration. If you leave this blank, this provider will auto-generate a unique name. Conflicts with `name_prefix`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// The tenancy of the instance. Valid values are `default` or `dedicated`, see [AWS's Create Launch Configuration](http://docs.aws.amazon.com/AutoScaling/latest/APIReference/API_CreateLaunchConfiguration.html) for more details.
        #[builder(into, default)]
        pub placement_tenancy: pulumi_wasm_rust::Output<Option<String>>,
        /// Customize details about the root block device of the instance. See Block Devices below for details.
        #[builder(into, default)]
        pub root_block_device: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::LaunchConfigurationRootBlockDevice>,
        >,
        /// A list of associated security group IDS.
        #[builder(into, default)]
        pub security_groups: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The maximum price to use for reserving spot instances.
        #[builder(into, default)]
        pub spot_price: pulumi_wasm_rust::Output<Option<String>>,
        /// The user data to provide when launching the instance. Do not pass gzip-compressed data via this argument; see `user_data_base64` instead.
        #[builder(into, default)]
        pub user_data: pulumi_wasm_rust::Output<Option<String>>,
        /// Can be used instead of `user_data` to pass base64-encoded binary data directly. Use this instead of `user_data` whenever the value is not a valid UTF-8 string. For example, gzip-encoded user data must be base64-encoded and passed via this argument to avoid corruption.
        #[builder(into, default)]
        pub user_data_base64: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LaunchConfigurationResult {
        /// The Amazon Resource Name of the launch configuration.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Associate a public ip address with an instance in a VPC.
        pub associate_public_ip_address: pulumi_wasm_rust::Output<Option<bool>>,
        /// Additional EBS block devices to attach to the instance. See Block Devices below for details.
        pub ebs_block_devices: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::LaunchConfigurationEbsBlockDevice>,
        >,
        /// If true, the launched EC2 instance will be EBS-optimized.
        pub ebs_optimized: pulumi_wasm_rust::Output<bool>,
        /// Enables/disables detailed monitoring. This is enabled by default.
        pub enable_monitoring: pulumi_wasm_rust::Output<Option<bool>>,
        /// Customize Ephemeral (also known as "Instance Store") volumes on the instance. See Block Devices below for details.
        pub ephemeral_block_devices: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::ec2::LaunchConfigurationEphemeralBlockDevice>,
            >,
        >,
        /// The name attribute of the IAM instance profile to associate with launched instances.
        pub iam_instance_profile: pulumi_wasm_rust::Output<Option<String>>,
        /// The EC2 image ID to launch.
        pub image_id: pulumi_wasm_rust::Output<String>,
        /// The size of instance to launch.
        ///
        /// The following arguments are optional:
        pub instance_type: pulumi_wasm_rust::Output<String>,
        /// The key name that should be used for the instance.
        pub key_name: pulumi_wasm_rust::Output<String>,
        /// The metadata options for the instance.
        pub metadata_options: pulumi_wasm_rust::Output<
            super::super::types::ec2::LaunchConfigurationMetadataOptions,
        >,
        /// The name of the launch configuration. If you leave this blank, this provider will auto-generate a unique name. Conflicts with `name_prefix`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// The tenancy of the instance. Valid values are `default` or `dedicated`, see [AWS's Create Launch Configuration](http://docs.aws.amazon.com/AutoScaling/latest/APIReference/API_CreateLaunchConfiguration.html) for more details.
        pub placement_tenancy: pulumi_wasm_rust::Output<Option<String>>,
        /// Customize details about the root block device of the instance. See Block Devices below for details.
        pub root_block_device: pulumi_wasm_rust::Output<
            super::super::types::ec2::LaunchConfigurationRootBlockDevice,
        >,
        /// A list of associated security group IDS.
        pub security_groups: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The maximum price to use for reserving spot instances.
        pub spot_price: pulumi_wasm_rust::Output<Option<String>>,
        /// The user data to provide when launching the instance. Do not pass gzip-compressed data via this argument; see `user_data_base64` instead.
        pub user_data: pulumi_wasm_rust::Output<Option<String>>,
        /// Can be used instead of `user_data` to pass base64-encoded binary data directly. Use this instead of `user_data` whenever the value is not a valid UTF-8 string. For example, gzip-encoded user data must be base64-encoded and passed via this argument to avoid corruption.
        pub user_data_base64: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: LaunchConfigurationArgs,
    ) -> LaunchConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let associate_public_ip_address_binding = args
            .associate_public_ip_address
            .get_inner();
        let ebs_block_devices_binding = args.ebs_block_devices.get_inner();
        let ebs_optimized_binding = args.ebs_optimized.get_inner();
        let enable_monitoring_binding = args.enable_monitoring.get_inner();
        let ephemeral_block_devices_binding = args.ephemeral_block_devices.get_inner();
        let iam_instance_profile_binding = args.iam_instance_profile.get_inner();
        let image_id_binding = args.image_id.get_inner();
        let instance_type_binding = args.instance_type.get_inner();
        let key_name_binding = args.key_name.get_inner();
        let metadata_options_binding = args.metadata_options.get_inner();
        let name_binding = args.name.get_inner();
        let name_prefix_binding = args.name_prefix.get_inner();
        let placement_tenancy_binding = args.placement_tenancy.get_inner();
        let root_block_device_binding = args.root_block_device.get_inner();
        let security_groups_binding = args.security_groups.get_inner();
        let spot_price_binding = args.spot_price.get_inner();
        let user_data_binding = args.user_data.get_inner();
        let user_data_base64_binding = args.user_data_base64.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/launchConfiguration:LaunchConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "associatePublicIpAddress".into(),
                    value: &associate_public_ip_address_binding,
                },
                register_interface::ObjectField {
                    name: "ebsBlockDevices".into(),
                    value: &ebs_block_devices_binding,
                },
                register_interface::ObjectField {
                    name: "ebsOptimized".into(),
                    value: &ebs_optimized_binding,
                },
                register_interface::ObjectField {
                    name: "enableMonitoring".into(),
                    value: &enable_monitoring_binding,
                },
                register_interface::ObjectField {
                    name: "ephemeralBlockDevices".into(),
                    value: &ephemeral_block_devices_binding,
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
                    name: "instanceType".into(),
                    value: &instance_type_binding,
                },
                register_interface::ObjectField {
                    name: "keyName".into(),
                    value: &key_name_binding,
                },
                register_interface::ObjectField {
                    name: "metadataOptions".into(),
                    value: &metadata_options_binding,
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
                    name: "placementTenancy".into(),
                    value: &placement_tenancy_binding,
                },
                register_interface::ObjectField {
                    name: "rootBlockDevice".into(),
                    value: &root_block_device_binding,
                },
                register_interface::ObjectField {
                    name: "securityGroups".into(),
                    value: &security_groups_binding,
                },
                register_interface::ObjectField {
                    name: "spotPrice".into(),
                    value: &spot_price_binding,
                },
                register_interface::ObjectField {
                    name: "userData".into(),
                    value: &user_data_binding,
                },
                register_interface::ObjectField {
                    name: "userDataBase64".into(),
                    value: &user_data_base64_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "associatePublicIpAddress".into(),
                },
                register_interface::ResultField {
                    name: "ebsBlockDevices".into(),
                },
                register_interface::ResultField {
                    name: "ebsOptimized".into(),
                },
                register_interface::ResultField {
                    name: "enableMonitoring".into(),
                },
                register_interface::ResultField {
                    name: "ephemeralBlockDevices".into(),
                },
                register_interface::ResultField {
                    name: "iamInstanceProfile".into(),
                },
                register_interface::ResultField {
                    name: "imageId".into(),
                },
                register_interface::ResultField {
                    name: "instanceType".into(),
                },
                register_interface::ResultField {
                    name: "keyName".into(),
                },
                register_interface::ResultField {
                    name: "metadataOptions".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
                },
                register_interface::ResultField {
                    name: "placementTenancy".into(),
                },
                register_interface::ResultField {
                    name: "rootBlockDevice".into(),
                },
                register_interface::ResultField {
                    name: "securityGroups".into(),
                },
                register_interface::ResultField {
                    name: "spotPrice".into(),
                },
                register_interface::ResultField {
                    name: "userData".into(),
                },
                register_interface::ResultField {
                    name: "userDataBase64".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LaunchConfigurationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            associate_public_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associatePublicIpAddress").unwrap(),
            ),
            ebs_block_devices: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsBlockDevices").unwrap(),
            ),
            ebs_optimized: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsOptimized").unwrap(),
            ),
            enable_monitoring: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableMonitoring").unwrap(),
            ),
            ephemeral_block_devices: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ephemeralBlockDevices").unwrap(),
            ),
            iam_instance_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamInstanceProfile").unwrap(),
            ),
            image_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageId").unwrap(),
            ),
            instance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceType").unwrap(),
            ),
            key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyName").unwrap(),
            ),
            metadata_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadataOptions").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
            ),
            placement_tenancy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("placementTenancy").unwrap(),
            ),
            root_block_device: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootBlockDevice").unwrap(),
            ),
            security_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroups").unwrap(),
            ),
            spot_price: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("spotPrice").unwrap(),
            ),
            user_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userData").unwrap(),
            ),
            user_data_base64: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userDataBase64").unwrap(),
            ),
        }
    }
}
