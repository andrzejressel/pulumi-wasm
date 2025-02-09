/// Manages a Shared Image within a Shared Image Gallery.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleSharedImageGallery:
///     type: azure:compute:SharedImageGallery
///     name: example
///     properties:
///       name: example_image_gallery
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       description: Shared images and things.
///       tags:
///         Hello: There
///         World: Example
///   exampleSharedImage:
///     type: azure:compute:SharedImage
///     name: example
///     properties:
///       name: my-image
///       galleryName: ${exampleSharedImageGallery.name}
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       osType: Linux
///       identifier:
///         publisher: PublisherName
///         offer: OfferName
///         sku: ExampleSku
/// ```
///
/// ## Import
///
/// Shared Images can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/sharedImage:SharedImage image1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/galleries/gallery1/images/image1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod shared_image {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SharedImageArgs {
        /// Specifies if the Shared Image supports Accelerated Network. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub accelerated_network_support_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// CPU architecture supported by an OS. Possible values are `x64` and `Arm64`. Defaults to `x64`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub architecture: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies if Confidential Virtual Machines enabled. It will enable all the features of trusted, with higher confidentiality features for isolate machines or encrypted data. Available for Gen2 machines. Changing this forces a new resource to be created.
        ///
        /// > **Note:**: Only one of `trusted_launch_supported`, `trusted_launch_enabled`, `confidential_vm_supported` and `confidential_vm_enabled` can be specified.
        #[builder(into, default)]
        pub confidential_vm_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies if supports creation of both Confidential virtual machines and Gen2 virtual machines with standard security from a compatible Gen2 OS disk VHD or Gen2 Managed image. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub confidential_vm_supported: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A description of this Shared Image.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies if the Shared Image supports NVMe disks. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub disk_controller_type_nvme_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// One or more Disk Types not allowed for the Image. Possible values include `Standard_LRS` and `Premium_LRS`.
        #[builder(into, default)]
        pub disk_types_not_alloweds: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The end of life date in RFC3339 format of the Image.
        #[builder(into, default)]
        pub end_of_life_date: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The End User Licence Agreement for the Shared Image. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub eula: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Shared Image Gallery in which this Shared Image should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub gallery_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies if the Shared Image supports hibernation. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub hibernation_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The generation of HyperV that the Virtual Machine used to create the Shared Image is based on. Possible values are `V1` and `V2`. Defaults to `V1`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub hyper_v_generation: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `identifier` block as defined below.
        #[builder(into)]
        pub identifier: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::SharedImageIdentifier,
        >,
        /// Specifies the supported Azure location where the Shared Image Gallery exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Maximum memory in GB recommended for the Image.
        #[builder(into, default)]
        pub max_recommended_memory_in_gb: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Maximum count of vCPUs recommended for the Image.
        #[builder(into, default)]
        pub max_recommended_vcpu_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Minimum memory in GB recommended for the Image.
        #[builder(into, default)]
        pub min_recommended_memory_in_gb: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Minimum count of vCPUs recommended for the Image.
        #[builder(into, default)]
        pub min_recommended_vcpu_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the name of the Shared Image. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of Operating System present in this Shared Image. Possible values are `Linux` and `Windows`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub os_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The URI containing the Privacy Statement associated with this Shared Image. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub privacy_statement_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `purchase_plan` block as defined below.
        #[builder(into, default)]
        pub purchase_plan: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::SharedImagePurchasePlan>,
        >,
        /// The URI containing the Release Notes associated with this Shared Image.
        #[builder(into, default)]
        pub release_note_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the Shared Image Gallery exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies that the Operating System used inside this Image has not been Generalized (for example, `sysprep` on Windows has not been run). Changing this forces a new resource to be created.
        ///
        /// !> **Note:** It's recommended to Generalize images where possible - Specialized Images reuse the same UUID internally within each Virtual Machine, which can have unintended side-effects.
        #[builder(into, default)]
        pub specialized: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A mapping of tags to assign to the Shared Image.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies if Trusted Launch has to be enabled for the Virtual Machine created from the Shared Image. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub trusted_launch_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies if supports creation of both Trusted Launch virtual machines and Gen2 virtual machines with standard security created from the Shared Image. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub trusted_launch_supported: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct SharedImageResult {
        /// Specifies if the Shared Image supports Accelerated Network. Changing this forces a new resource to be created.
        pub accelerated_network_support_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// CPU architecture supported by an OS. Possible values are `x64` and `Arm64`. Defaults to `x64`. Changing this forces a new resource to be created.
        pub architecture: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies if Confidential Virtual Machines enabled. It will enable all the features of trusted, with higher confidentiality features for isolate machines or encrypted data. Available for Gen2 machines. Changing this forces a new resource to be created.
        ///
        /// > **Note:**: Only one of `trusted_launch_supported`, `trusted_launch_enabled`, `confidential_vm_supported` and `confidential_vm_enabled` can be specified.
        pub confidential_vm_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies if supports creation of both Confidential virtual machines and Gen2 virtual machines with standard security from a compatible Gen2 OS disk VHD or Gen2 Managed image. Changing this forces a new resource to be created.
        pub confidential_vm_supported: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A description of this Shared Image.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies if the Shared Image supports NVMe disks. Changing this forces a new resource to be created.
        pub disk_controller_type_nvme_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// One or more Disk Types not allowed for the Image. Possible values include `Standard_LRS` and `Premium_LRS`.
        pub disk_types_not_alloweds: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The end of life date in RFC3339 format of the Image.
        pub end_of_life_date: pulumi_gestalt_rust::Output<Option<String>>,
        /// The End User Licence Agreement for the Shared Image. Changing this forces a new resource to be created.
        pub eula: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Shared Image Gallery in which this Shared Image should exist. Changing this forces a new resource to be created.
        pub gallery_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies if the Shared Image supports hibernation. Changing this forces a new resource to be created.
        pub hibernation_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The generation of HyperV that the Virtual Machine used to create the Shared Image is based on. Possible values are `V1` and `V2`. Defaults to `V1`. Changing this forces a new resource to be created.
        pub hyper_v_generation: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `identifier` block as defined below.
        pub identifier: pulumi_gestalt_rust::Output<
            super::super::types::compute::SharedImageIdentifier,
        >,
        /// Specifies the supported Azure location where the Shared Image Gallery exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Maximum memory in GB recommended for the Image.
        pub max_recommended_memory_in_gb: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Maximum count of vCPUs recommended for the Image.
        pub max_recommended_vcpu_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Minimum memory in GB recommended for the Image.
        pub min_recommended_memory_in_gb: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Minimum count of vCPUs recommended for the Image.
        pub min_recommended_vcpu_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the name of the Shared Image. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The type of Operating System present in this Shared Image. Possible values are `Linux` and `Windows`. Changing this forces a new resource to be created.
        pub os_type: pulumi_gestalt_rust::Output<String>,
        /// The URI containing the Privacy Statement associated with this Shared Image. Changing this forces a new resource to be created.
        pub privacy_statement_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `purchase_plan` block as defined below.
        pub purchase_plan: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::SharedImagePurchasePlan>,
        >,
        /// The URI containing the Release Notes associated with this Shared Image.
        pub release_note_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group in which the Shared Image Gallery exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies that the Operating System used inside this Image has not been Generalized (for example, `sysprep` on Windows has not been run). Changing this forces a new resource to be created.
        ///
        /// !> **Note:** It's recommended to Generalize images where possible - Specialized Images reuse the same UUID internally within each Virtual Machine, which can have unintended side-effects.
        pub specialized: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A mapping of tags to assign to the Shared Image.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies if Trusted Launch has to be enabled for the Virtual Machine created from the Shared Image. Changing this forces a new resource to be created.
        pub trusted_launch_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies if supports creation of both Trusted Launch virtual machines and Gen2 virtual machines with standard security created from the Shared Image. Changing this forces a new resource to be created.
        pub trusted_launch_supported: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SharedImageArgs,
    ) -> SharedImageResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let accelerated_network_support_enabled_binding_1 = args
            .accelerated_network_support_enabled
            .get_output(context);
        let accelerated_network_support_enabled_binding = accelerated_network_support_enabled_binding_1
            .get_inner();
        let architecture_binding_1 = args.architecture.get_output(context);
        let architecture_binding = architecture_binding_1.get_inner();
        let confidential_vm_enabled_binding_1 = args
            .confidential_vm_enabled
            .get_output(context);
        let confidential_vm_enabled_binding = confidential_vm_enabled_binding_1
            .get_inner();
        let confidential_vm_supported_binding_1 = args
            .confidential_vm_supported
            .get_output(context);
        let confidential_vm_supported_binding = confidential_vm_supported_binding_1
            .get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let disk_controller_type_nvme_enabled_binding_1 = args
            .disk_controller_type_nvme_enabled
            .get_output(context);
        let disk_controller_type_nvme_enabled_binding = disk_controller_type_nvme_enabled_binding_1
            .get_inner();
        let disk_types_not_alloweds_binding_1 = args
            .disk_types_not_alloweds
            .get_output(context);
        let disk_types_not_alloweds_binding = disk_types_not_alloweds_binding_1
            .get_inner();
        let end_of_life_date_binding_1 = args.end_of_life_date.get_output(context);
        let end_of_life_date_binding = end_of_life_date_binding_1.get_inner();
        let eula_binding_1 = args.eula.get_output(context);
        let eula_binding = eula_binding_1.get_inner();
        let gallery_name_binding_1 = args.gallery_name.get_output(context);
        let gallery_name_binding = gallery_name_binding_1.get_inner();
        let hibernation_enabled_binding_1 = args.hibernation_enabled.get_output(context);
        let hibernation_enabled_binding = hibernation_enabled_binding_1.get_inner();
        let hyper_v_generation_binding_1 = args.hyper_v_generation.get_output(context);
        let hyper_v_generation_binding = hyper_v_generation_binding_1.get_inner();
        let identifier_binding_1 = args.identifier.get_output(context);
        let identifier_binding = identifier_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let max_recommended_memory_in_gb_binding_1 = args
            .max_recommended_memory_in_gb
            .get_output(context);
        let max_recommended_memory_in_gb_binding = max_recommended_memory_in_gb_binding_1
            .get_inner();
        let max_recommended_vcpu_count_binding_1 = args
            .max_recommended_vcpu_count
            .get_output(context);
        let max_recommended_vcpu_count_binding = max_recommended_vcpu_count_binding_1
            .get_inner();
        let min_recommended_memory_in_gb_binding_1 = args
            .min_recommended_memory_in_gb
            .get_output(context);
        let min_recommended_memory_in_gb_binding = min_recommended_memory_in_gb_binding_1
            .get_inner();
        let min_recommended_vcpu_count_binding_1 = args
            .min_recommended_vcpu_count
            .get_output(context);
        let min_recommended_vcpu_count_binding = min_recommended_vcpu_count_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let os_type_binding_1 = args.os_type.get_output(context);
        let os_type_binding = os_type_binding_1.get_inner();
        let privacy_statement_uri_binding_1 = args
            .privacy_statement_uri
            .get_output(context);
        let privacy_statement_uri_binding = privacy_statement_uri_binding_1.get_inner();
        let purchase_plan_binding_1 = args.purchase_plan.get_output(context);
        let purchase_plan_binding = purchase_plan_binding_1.get_inner();
        let release_note_uri_binding_1 = args.release_note_uri.get_output(context);
        let release_note_uri_binding = release_note_uri_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let specialized_binding_1 = args.specialized.get_output(context);
        let specialized_binding = specialized_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let trusted_launch_enabled_binding_1 = args
            .trusted_launch_enabled
            .get_output(context);
        let trusted_launch_enabled_binding = trusted_launch_enabled_binding_1
            .get_inner();
        let trusted_launch_supported_binding_1 = args
            .trusted_launch_supported
            .get_output(context);
        let trusted_launch_supported_binding = trusted_launch_supported_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/sharedImage:SharedImage".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "acceleratedNetworkSupportEnabled".into(),
                    value: &accelerated_network_support_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "architecture".into(),
                    value: &architecture_binding,
                },
                register_interface::ObjectField {
                    name: "confidentialVmEnabled".into(),
                    value: &confidential_vm_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "confidentialVmSupported".into(),
                    value: &confidential_vm_supported_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "diskControllerTypeNvmeEnabled".into(),
                    value: &disk_controller_type_nvme_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "diskTypesNotAlloweds".into(),
                    value: &disk_types_not_alloweds_binding,
                },
                register_interface::ObjectField {
                    name: "endOfLifeDate".into(),
                    value: &end_of_life_date_binding,
                },
                register_interface::ObjectField {
                    name: "eula".into(),
                    value: &eula_binding,
                },
                register_interface::ObjectField {
                    name: "galleryName".into(),
                    value: &gallery_name_binding,
                },
                register_interface::ObjectField {
                    name: "hibernationEnabled".into(),
                    value: &hibernation_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "hyperVGeneration".into(),
                    value: &hyper_v_generation_binding,
                },
                register_interface::ObjectField {
                    name: "identifier".into(),
                    value: &identifier_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "maxRecommendedMemoryInGb".into(),
                    value: &max_recommended_memory_in_gb_binding,
                },
                register_interface::ObjectField {
                    name: "maxRecommendedVcpuCount".into(),
                    value: &max_recommended_vcpu_count_binding,
                },
                register_interface::ObjectField {
                    name: "minRecommendedMemoryInGb".into(),
                    value: &min_recommended_memory_in_gb_binding,
                },
                register_interface::ObjectField {
                    name: "minRecommendedVcpuCount".into(),
                    value: &min_recommended_vcpu_count_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "osType".into(),
                    value: &os_type_binding,
                },
                register_interface::ObjectField {
                    name: "privacyStatementUri".into(),
                    value: &privacy_statement_uri_binding,
                },
                register_interface::ObjectField {
                    name: "purchasePlan".into(),
                    value: &purchase_plan_binding,
                },
                register_interface::ObjectField {
                    name: "releaseNoteUri".into(),
                    value: &release_note_uri_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "specialized".into(),
                    value: &specialized_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "trustedLaunchEnabled".into(),
                    value: &trusted_launch_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "trustedLaunchSupported".into(),
                    value: &trusted_launch_supported_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SharedImageResult {
            accelerated_network_support_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("acceleratedNetworkSupportEnabled"),
            ),
            architecture: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("architecture"),
            ),
            confidential_vm_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("confidentialVmEnabled"),
            ),
            confidential_vm_supported: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("confidentialVmSupported"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            disk_controller_type_nvme_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskControllerTypeNvmeEnabled"),
            ),
            disk_types_not_alloweds: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskTypesNotAlloweds"),
            ),
            end_of_life_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endOfLifeDate"),
            ),
            eula: pulumi_gestalt_rust::__private::into_domain(o.extract_field("eula")),
            gallery_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("galleryName"),
            ),
            hibernation_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hibernationEnabled"),
            ),
            hyper_v_generation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hyperVGeneration"),
            ),
            identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identifier"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            max_recommended_memory_in_gb: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxRecommendedMemoryInGb"),
            ),
            max_recommended_vcpu_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxRecommendedVcpuCount"),
            ),
            min_recommended_memory_in_gb: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minRecommendedMemoryInGb"),
            ),
            min_recommended_vcpu_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minRecommendedVcpuCount"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            os_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("osType"),
            ),
            privacy_statement_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privacyStatementUri"),
            ),
            purchase_plan: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("purchasePlan"),
            ),
            release_note_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("releaseNoteUri"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            specialized: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("specialized"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            trusted_launch_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("trustedLaunchEnabled"),
            ),
            trusted_launch_supported: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("trustedLaunchSupported"),
            ),
        }
    }
}
