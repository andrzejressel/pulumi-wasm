#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_launch_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLaunchTemplateArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetLaunchTemplateFilter>>,
        >,
        /// ID of the specific launch template to retrieve.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the launch template.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags, each pair of which must exactly match a pair on the desired Launch Template.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLaunchTemplateResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub block_device_mappings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateBlockDeviceMapping>,
        >,
        pub capacity_reservation_specifications: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::ec2::GetLaunchTemplateCapacityReservationSpecification,
            >,
        >,
        pub cpu_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateCpuOption>,
        >,
        pub credit_specifications: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateCreditSpecification>,
        >,
        pub default_version: pulumi_gestalt_rust::Output<i32>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub disable_api_stop: pulumi_gestalt_rust::Output<bool>,
        pub disable_api_termination: pulumi_gestalt_rust::Output<bool>,
        pub ebs_optimized: pulumi_gestalt_rust::Output<String>,
        pub elastic_gpu_specifications: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::ec2::GetLaunchTemplateElasticGpuSpecification,
            >,
        >,
        pub elastic_inference_accelerators: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::ec2::GetLaunchTemplateElasticInferenceAccelerator,
            >,
        >,
        pub enclave_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateEnclaveOption>,
        >,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetLaunchTemplateFilter>>,
        >,
        pub hibernation_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateHibernationOption>,
        >,
        pub iam_instance_profiles: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateIamInstanceProfile>,
        >,
        /// ID of the launch template.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub image_id: pulumi_gestalt_rust::Output<String>,
        pub instance_initiated_shutdown_behavior: pulumi_gestalt_rust::Output<String>,
        pub instance_market_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateInstanceMarketOption>,
        >,
        pub instance_requirements: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateInstanceRequirement>,
        >,
        pub instance_type: pulumi_gestalt_rust::Output<String>,
        pub kernel_id: pulumi_gestalt_rust::Output<String>,
        pub key_name: pulumi_gestalt_rust::Output<String>,
        pub latest_version: pulumi_gestalt_rust::Output<i32>,
        pub license_specifications: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateLicenseSpecification>,
        >,
        pub maintenance_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateMaintenanceOption>,
        >,
        pub metadata_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateMetadataOption>,
        >,
        pub monitorings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateMonitoring>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub network_interfaces: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateNetworkInterface>,
        >,
        pub placements: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplatePlacement>,
        >,
        pub private_dns_name_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplatePrivateDnsNameOption>,
        >,
        pub ram_disk_id: pulumi_gestalt_rust::Output<String>,
        pub security_group_names: pulumi_gestalt_rust::Output<Vec<String>>,
        pub tag_specifications: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetLaunchTemplateTagSpecification>,
        >,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub user_data: pulumi_gestalt_rust::Output<String>,
        pub vpc_security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetLaunchTemplateArgs,
    ) -> GetLaunchTemplateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let id_binding = args.id.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getLaunchTemplate:getLaunchTemplate".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: &id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetLaunchTemplateResult {
            arn: o.get_field("arn"),
            block_device_mappings: o.get_field("blockDeviceMappings"),
            capacity_reservation_specifications: o
                .get_field("capacityReservationSpecifications"),
            cpu_options: o.get_field("cpuOptions"),
            credit_specifications: o.get_field("creditSpecifications"),
            default_version: o.get_field("defaultVersion"),
            description: o.get_field("description"),
            disable_api_stop: o.get_field("disableApiStop"),
            disable_api_termination: o.get_field("disableApiTermination"),
            ebs_optimized: o.get_field("ebsOptimized"),
            elastic_gpu_specifications: o.get_field("elasticGpuSpecifications"),
            elastic_inference_accelerators: o.get_field("elasticInferenceAccelerators"),
            enclave_options: o.get_field("enclaveOptions"),
            filters: o.get_field("filters"),
            hibernation_options: o.get_field("hibernationOptions"),
            iam_instance_profiles: o.get_field("iamInstanceProfiles"),
            id: o.get_field("id"),
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
            monitorings: o.get_field("monitorings"),
            name: o.get_field("name"),
            network_interfaces: o.get_field("networkInterfaces"),
            placements: o.get_field("placements"),
            private_dns_name_options: o.get_field("privateDnsNameOptions"),
            ram_disk_id: o.get_field("ramDiskId"),
            security_group_names: o.get_field("securityGroupNames"),
            tag_specifications: o.get_field("tagSpecifications"),
            tags: o.get_field("tags"),
            user_data: o.get_field("userData"),
            vpc_security_group_ids: o.get_field("vpcSecurityGroupIds"),
        }
    }
}
