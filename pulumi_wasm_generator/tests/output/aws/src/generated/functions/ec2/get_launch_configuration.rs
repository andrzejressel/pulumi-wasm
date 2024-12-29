pub mod get_launch_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLaunchConfigurationArgs {
        /// Name of the launch configuration.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetLaunchConfigurationResult {
        /// Amazon Resource Name of the launch configuration.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Whether a Public IP address is associated with the instance.
        pub associate_public_ip_address: pulumi_wasm_rust::Output<bool>,
        /// EBS Block Devices attached to the instance.
        pub ebs_block_devices: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchConfigurationEbsBlockDevice>,
        >,
        /// Whether the launched EC2 instance will be EBS-optimized.
        pub ebs_optimized: pulumi_wasm_rust::Output<bool>,
        /// Whether Detailed Monitoring is Enabled.
        pub enable_monitoring: pulumi_wasm_rust::Output<bool>,
        /// The Ephemeral volumes on the instance.
        pub ephemeral_block_devices: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::ec2::GetLaunchConfigurationEphemeralBlockDevice,
            >,
        >,
        /// The IAM Instance Profile to associate with launched instances.
        pub iam_instance_profile: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// EC2 Image ID of the instance.
        pub image_id: pulumi_wasm_rust::Output<String>,
        /// Instance Type of the instance to launch.
        pub instance_type: pulumi_wasm_rust::Output<String>,
        /// Key Name that should be used for the instance.
        pub key_name: pulumi_wasm_rust::Output<String>,
        /// Metadata options for the instance.
        pub metadata_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchConfigurationMetadataOption>,
        >,
        /// Name of the launch configuration.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Tenancy of the instance.
        pub placement_tenancy: pulumi_wasm_rust::Output<String>,
        /// Root Block Device of the instance.
        pub root_block_devices: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchConfigurationRootBlockDevice>,
        >,
        /// List of associated Security Group IDS.
        pub security_groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// Price to use for reserving Spot instances.
        pub spot_price: pulumi_wasm_rust::Output<String>,
        /// User Data of the instance.
        pub user_data: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetLaunchConfigurationArgs) -> GetLaunchConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getLaunchConfiguration:getLaunchConfiguration".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "id".into(),
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
                    name: "placementTenancy".into(),
                },
                register_interface::ResultField {
                    name: "rootBlockDevices".into(),
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
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetLaunchConfigurationResult {
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
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
            placement_tenancy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("placementTenancy").unwrap(),
            ),
            root_block_devices: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootBlockDevices").unwrap(),
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
        }
    }
}
