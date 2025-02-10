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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LaunchConfigurationArgs,
    ) -> LaunchConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let associate_public_ip_address_binding = args
            .associate_public_ip_address
            .get_output(context);
        let ebs_block_devices_binding = args.ebs_block_devices.get_output(context);
        let ebs_optimized_binding = args.ebs_optimized.get_output(context);
        let enable_monitoring_binding = args.enable_monitoring.get_output(context);
        let ephemeral_block_devices_binding = args
            .ephemeral_block_devices
            .get_output(context);
        let iam_instance_profile_binding = args.iam_instance_profile.get_output(context);
        let image_id_binding = args.image_id.get_output(context);
        let instance_type_binding = args.instance_type.get_output(context);
        let key_name_binding = args.key_name.get_output(context);
        let metadata_options_binding = args.metadata_options.get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let placement_tenancy_binding = args.placement_tenancy.get_output(context);
        let root_block_device_binding = args.root_block_device.get_output(context);
        let security_groups_binding = args.security_groups.get_output(context);
        let spot_price_binding = args.spot_price.get_output(context);
        let user_data_binding = args.user_data.get_output(context);
        let user_data_base64_binding = args.user_data_base64.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/launchConfiguration:LaunchConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "associatePublicIpAddress".into(),
                    value: associate_public_ip_address_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ebsBlockDevices".into(),
                    value: ebs_block_devices_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ebsOptimized".into(),
                    value: ebs_optimized_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableMonitoring".into(),
                    value: enable_monitoring_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ephemeralBlockDevices".into(),
                    value: ephemeral_block_devices_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iamInstanceProfile".into(),
                    value: iam_instance_profile_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageId".into(),
                    value: image_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceType".into(),
                    value: instance_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyName".into(),
                    value: key_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadataOptions".into(),
                    value: metadata_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: name_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "placementTenancy".into(),
                    value: placement_tenancy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rootBlockDevice".into(),
                    value: root_block_device_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityGroups".into(),
                    value: security_groups_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "spotPrice".into(),
                    value: spot_price_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userData".into(),
                    value: user_data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userDataBase64".into(),
                    value: user_data_base64_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LaunchConfigurationResult {
            arn: o.get_field("arn"),
            associate_public_ip_address: o.get_field("associatePublicIpAddress"),
            ebs_block_devices: o.get_field("ebsBlockDevices"),
            ebs_optimized: o.get_field("ebsOptimized"),
            enable_monitoring: o.get_field("enableMonitoring"),
            ephemeral_block_devices: o.get_field("ephemeralBlockDevices"),
            iam_instance_profile: o.get_field("iamInstanceProfile"),
            image_id: o.get_field("imageId"),
            instance_type: o.get_field("instanceType"),
            key_name: o.get_field("keyName"),
            metadata_options: o.get_field("metadataOptions"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            placement_tenancy: o.get_field("placementTenancy"),
            root_block_device: o.get_field("rootBlockDevice"),
            security_groups: o.get_field("securityGroups"),
            spot_price: o.get_field("spotPrice"),
            user_data: o.get_field("userData"),
            user_data_base64: o.get_field("userDataBase64"),
        }
    }
}
