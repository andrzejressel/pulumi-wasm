pub mod get_launch_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLaunchConfigurationArgs {
        /// Name of the launch configuration.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetLaunchConfigurationResult {
        /// Amazon Resource Name of the launch configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Whether a Public IP address is associated with the instance.
        pub associate_public_ip_address: pulumi_gestalt_rust::Output<bool>,
        /// EBS Block Devices attached to the instance.
        pub ebs_block_devices: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchConfigurationEbsBlockDevice>,
        >,
        /// Whether the launched EC2 instance will be EBS-optimized.
        pub ebs_optimized: pulumi_gestalt_rust::Output<bool>,
        /// Whether Detailed Monitoring is Enabled.
        pub enable_monitoring: pulumi_gestalt_rust::Output<bool>,
        /// The Ephemeral volumes on the instance.
        pub ephemeral_block_devices: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::ec2::GetLaunchConfigurationEphemeralBlockDevice,
            >,
        >,
        /// The IAM Instance Profile to associate with launched instances.
        pub iam_instance_profile: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// EC2 Image ID of the instance.
        pub image_id: pulumi_gestalt_rust::Output<String>,
        /// Instance Type of the instance to launch.
        pub instance_type: pulumi_gestalt_rust::Output<String>,
        /// Key Name that should be used for the instance.
        pub key_name: pulumi_gestalt_rust::Output<String>,
        /// Metadata options for the instance.
        pub metadata_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchConfigurationMetadataOption>,
        >,
        /// Name of the launch configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Tenancy of the instance.
        pub placement_tenancy: pulumi_gestalt_rust::Output<String>,
        /// Root Block Device of the instance.
        pub root_block_devices: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchConfigurationRootBlockDevice>,
        >,
        /// List of associated Security Group IDS.
        pub security_groups: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Price to use for reserving Spot instances.
        pub spot_price: pulumi_gestalt_rust::Output<String>,
        /// User Data of the instance.
        pub user_data: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetLaunchConfigurationArgs,
    ) -> GetLaunchConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getLaunchConfiguration:getLaunchConfiguration".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLaunchConfigurationResult {
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
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
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
            placement_tenancy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("placementTenancy"),
            ),
            root_block_devices: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rootBlockDevices"),
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
        }
    }
}
