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
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LaunchConfigurationArgs {
        /// Associate a public ip address with an instance in a VPC.
        #[builder(into, default)]
        pub associate_public_ip_address: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Additional EBS block devices to attach to the instance. See Block Devices below for details.
        #[builder(into, default)]
        pub ebs_block_devices: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::LaunchConfigurationEbsBlockDevice>>,
        >,
        /// If true, the launched EC2 instance will be EBS-optimized.
        #[builder(into, default)]
        pub ebs_optimized: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Enables/disables detailed monitoring. This is enabled by default.
        #[builder(into, default)]
        pub enable_monitoring: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Customize Ephemeral (also known as "Instance Store") volumes on the instance. See Block Devices below for details.
        #[builder(into, default)]
        pub ephemeral_block_devices: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::ec2::LaunchConfigurationEphemeralBlockDevice>,
            >,
        >,
        /// The name attribute of the IAM instance profile to associate with launched instances.
        #[builder(into, default)]
        pub iam_instance_profile: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The EC2 image ID to launch.
        #[builder(into)]
        pub image_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The size of instance to launch.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub instance_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The key name that should be used for the instance.
        #[builder(into, default)]
        pub key_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The metadata options for the instance.
        #[builder(into, default)]
        pub metadata_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::LaunchConfigurationMetadataOptions>,
        >,
        /// The name of the launch configuration. If you leave this blank, this provider will auto-generate a unique name. Conflicts with `name_prefix`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The tenancy of the instance. Valid values are `default` or `dedicated`, see [AWS's Create Launch Configuration](http://docs.aws.amazon.com/AutoScaling/latest/APIReference/API_CreateLaunchConfiguration.html) for more details.
        #[builder(into, default)]
        pub placement_tenancy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Customize details about the root block device of the instance. See Block Devices below for details.
        #[builder(into, default)]
        pub root_block_device: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::LaunchConfigurationRootBlockDevice>,
        >,
        /// A list of associated security group IDS.
        #[builder(into, default)]
        pub security_groups: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The maximum price to use for reserving spot instances.
        #[builder(into, default)]
        pub spot_price: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The user data to provide when launching the instance. Do not pass gzip-compressed data via this argument; see `user_data_base64` instead.
        #[builder(into, default)]
        pub user_data: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Can be used instead of `user_data` to pass base64-encoded binary data directly. Use this instead of `user_data` whenever the value is not a valid UTF-8 string. For example, gzip-encoded user data must be base64-encoded and passed via this argument to avoid corruption.
        #[builder(into, default)]
        pub user_data_base64: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LaunchConfigurationResult {
        /// The Amazon Resource Name of the launch configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Associate a public ip address with an instance in a VPC.
        pub associate_public_ip_address: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Additional EBS block devices to attach to the instance. See Block Devices below for details.
        pub ebs_block_devices: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::LaunchConfigurationEbsBlockDevice>,
        >,
        /// If true, the launched EC2 instance will be EBS-optimized.
        pub ebs_optimized: pulumi_gestalt_rust::Output<bool>,
        /// Enables/disables detailed monitoring. This is enabled by default.
        pub enable_monitoring: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Customize Ephemeral (also known as "Instance Store") volumes on the instance. See Block Devices below for details.
        pub ephemeral_block_devices: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::ec2::LaunchConfigurationEphemeralBlockDevice>,
            >,
        >,
        /// The name attribute of the IAM instance profile to associate with launched instances.
        pub iam_instance_profile: pulumi_gestalt_rust::Output<Option<String>>,
        /// The EC2 image ID to launch.
        pub image_id: pulumi_gestalt_rust::Output<String>,
        /// The size of instance to launch.
        ///
        /// The following arguments are optional:
        pub instance_type: pulumi_gestalt_rust::Output<String>,
        /// The key name that should be used for the instance.
        pub key_name: pulumi_gestalt_rust::Output<String>,
        /// The metadata options for the instance.
        pub metadata_options: pulumi_gestalt_rust::Output<
            super::super::types::ec2::LaunchConfigurationMetadataOptions,
        >,
        /// The name of the launch configuration. If you leave this blank, this provider will auto-generate a unique name. Conflicts with `name_prefix`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// The tenancy of the instance. Valid values are `default` or `dedicated`, see [AWS's Create Launch Configuration](http://docs.aws.amazon.com/AutoScaling/latest/APIReference/API_CreateLaunchConfiguration.html) for more details.
        pub placement_tenancy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Customize details about the root block device of the instance. See Block Devices below for details.
        pub root_block_device: pulumi_gestalt_rust::Output<
            super::super::types::ec2::LaunchConfigurationRootBlockDevice,
        >,
        /// A list of associated security group IDS.
        pub security_groups: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The maximum price to use for reserving spot instances.
        pub spot_price: pulumi_gestalt_rust::Output<Option<String>>,
        /// The user data to provide when launching the instance. Do not pass gzip-compressed data via this argument; see `user_data_base64` instead.
        pub user_data: pulumi_gestalt_rust::Output<Option<String>>,
        /// Can be used instead of `user_data` to pass base64-encoded binary data directly. Use this instead of `user_data` whenever the value is not a valid UTF-8 string. For example, gzip-encoded user data must be base64-encoded and passed via this argument to avoid corruption.
        pub user_data_base64: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: LaunchConfigurationArgs,
    ) -> LaunchConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let associate_public_ip_address_binding = args
            .associate_public_ip_address
            .get_output(context)
            .get_inner();
        let ebs_block_devices_binding = args
            .ebs_block_devices
            .get_output(context)
            .get_inner();
        let ebs_optimized_binding = args.ebs_optimized.get_output(context).get_inner();
        let enable_monitoring_binding = args
            .enable_monitoring
            .get_output(context)
            .get_inner();
        let ephemeral_block_devices_binding = args
            .ephemeral_block_devices
            .get_output(context)
            .get_inner();
        let iam_instance_profile_binding = args
            .iam_instance_profile
            .get_output(context)
            .get_inner();
        let image_id_binding = args.image_id.get_output(context).get_inner();
        let instance_type_binding = args.instance_type.get_output(context).get_inner();
        let key_name_binding = args.key_name.get_output(context).get_inner();
        let metadata_options_binding = args
            .metadata_options
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let name_prefix_binding = args.name_prefix.get_output(context).get_inner();
        let placement_tenancy_binding = args
            .placement_tenancy
            .get_output(context)
            .get_inner();
        let root_block_device_binding = args
            .root_block_device
            .get_output(context)
            .get_inner();
        let security_groups_binding = args
            .security_groups
            .get_output(context)
            .get_inner();
        let spot_price_binding = args.spot_price.get_output(context).get_inner();
        let user_data_binding = args.user_data.get_output(context).get_inner();
        let user_data_base64_binding = args
            .user_data_base64
            .get_output(context)
            .get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        LaunchConfigurationResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            associate_public_ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("associatePublicIpAddress"),
            ),
            ebs_block_devices: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ebsBlockDevices"),
            ),
            ebs_optimized: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ebsOptimized"),
            ),
            enable_monitoring: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableMonitoring"),
            ),
            ephemeral_block_devices: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ephemeralBlockDevices"),
            ),
            iam_instance_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iamInstanceProfile"),
            ),
            image_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageId"),
            ),
            instance_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceType"),
            ),
            key_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyName"),
            ),
            metadata_options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadataOptions"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            name_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namePrefix"),
            ),
            placement_tenancy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("placementTenancy"),
            ),
            root_block_device: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rootBlockDevice"),
            ),
            security_groups: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroups"),
            ),
            spot_price: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("spotPrice"),
            ),
            user_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userData"),
            ),
            user_data_base64: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userDataBase64"),
            ),
        }
    }
}
