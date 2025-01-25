pub mod get_launch_template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLaunchTemplateArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetLaunchTemplateFilter>>,
        >,
        /// ID of the specific launch template to retrieve.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the launch template.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags, each pair of which must exactly match a pair on the desired Launch Template.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLaunchTemplateResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        pub block_device_mappings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateBlockDeviceMapping>,
        >,
        pub capacity_reservation_specifications: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::ec2::GetLaunchTemplateCapacityReservationSpecification,
            >,
        >,
        pub cpu_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateCpuOption>,
        >,
        pub credit_specifications: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateCreditSpecification>,
        >,
        pub default_version: pulumi_wasm_rust::Output<i32>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub disable_api_stop: pulumi_wasm_rust::Output<bool>,
        pub disable_api_termination: pulumi_wasm_rust::Output<bool>,
        pub ebs_optimized: pulumi_wasm_rust::Output<String>,
        pub elastic_gpu_specifications: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::ec2::GetLaunchTemplateElasticGpuSpecification,
            >,
        >,
        pub elastic_inference_accelerators: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::ec2::GetLaunchTemplateElasticInferenceAccelerator,
            >,
        >,
        pub enclave_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateEnclaveOption>,
        >,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetLaunchTemplateFilter>>,
        >,
        pub hibernation_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateHibernationOption>,
        >,
        pub iam_instance_profiles: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateIamInstanceProfile>,
        >,
        /// ID of the launch template.
        pub id: pulumi_wasm_rust::Output<String>,
        pub image_id: pulumi_wasm_rust::Output<String>,
        pub instance_initiated_shutdown_behavior: pulumi_wasm_rust::Output<String>,
        pub instance_market_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateInstanceMarketOption>,
        >,
        pub instance_requirements: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateInstanceRequirement>,
        >,
        pub instance_type: pulumi_wasm_rust::Output<String>,
        pub kernel_id: pulumi_wasm_rust::Output<String>,
        pub key_name: pulumi_wasm_rust::Output<String>,
        pub latest_version: pulumi_wasm_rust::Output<i32>,
        pub license_specifications: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateLicenseSpecification>,
        >,
        pub maintenance_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateMaintenanceOption>,
        >,
        pub metadata_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateMetadataOption>,
        >,
        pub monitorings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateMonitoring>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub network_interfaces: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateNetworkInterface>,
        >,
        pub placements: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplatePlacement>,
        >,
        pub private_dns_name_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplatePrivateDnsNameOption>,
        >,
        pub ram_disk_id: pulumi_wasm_rust::Output<String>,
        pub security_group_names: pulumi_wasm_rust::Output<Vec<String>>,
        pub tag_specifications: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateTagSpecification>,
        >,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub user_data: pulumi_wasm_rust::Output<String>,
        pub vpc_security_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetLaunchTemplateArgs,
    ) -> GetLaunchTemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let id_binding = args.id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getLaunchTemplate:getLaunchTemplate".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
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
                    name: "capacityReservationSpecifications".into(),
                },
                register_interface::ResultField {
                    name: "cpuOptions".into(),
                },
                register_interface::ResultField {
                    name: "creditSpecifications".into(),
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
                    name: "elasticInferenceAccelerators".into(),
                },
                register_interface::ResultField {
                    name: "enclaveOptions".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "hibernationOptions".into(),
                },
                register_interface::ResultField {
                    name: "iamInstanceProfiles".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
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
                    name: "monitorings".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaces".into(),
                },
                register_interface::ResultField {
                    name: "placements".into(),
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
                    name: "userData".into(),
                },
                register_interface::ResultField {
                    name: "vpcSecurityGroupIds".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetLaunchTemplateResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            block_device_mappings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blockDeviceMappings").unwrap(),
            ),
            capacity_reservation_specifications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacityReservationSpecifications").unwrap(),
            ),
            cpu_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cpuOptions").unwrap(),
            ),
            credit_specifications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creditSpecifications").unwrap(),
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
            elastic_inference_accelerators: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("elasticInferenceAccelerators").unwrap(),
            ),
            enclave_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enclaveOptions").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            hibernation_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hibernationOptions").unwrap(),
            ),
            iam_instance_profiles: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamInstanceProfiles").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
            monitorings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monitorings").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_interfaces: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaces").unwrap(),
            ),
            placements: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("placements").unwrap(),
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
            user_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userData").unwrap(),
            ),
            vpc_security_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcSecurityGroupIds").unwrap(),
            ),
        }
    }
}
