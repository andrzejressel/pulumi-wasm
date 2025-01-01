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
pub mod shared_image {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SharedImageArgs {
        /// Specifies if the Shared Image supports Accelerated Network. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub accelerated_network_support_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// CPU architecture supported by an OS. Possible values are `x64` and `Arm64`. Defaults to `x64`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub architecture: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies if Confidential Virtual Machines enabled. It will enable all the features of trusted, with higher confidentiality features for isolate machines or encrypted data. Available for Gen2 machines. Changing this forces a new resource to be created.
        ///
        /// > **Note:**: Only one of `trusted_launch_supported`, `trusted_launch_enabled`, `confidential_vm_supported` and `confidential_vm_enabled` can be specified.
        #[builder(into, default)]
        pub confidential_vm_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies if supports creation of both Confidential virtual machines and Gen2 virtual machines with standard security from a compatible Gen2 OS disk VHD or Gen2 Managed image. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub confidential_vm_supported: pulumi_wasm_rust::Output<Option<bool>>,
        /// A description of this Shared Image.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies if the Shared Image supports NVMe disks. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub disk_controller_type_nvme_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more Disk Types not allowed for the Image. Possible values include `Standard_LRS` and `Premium_LRS`.
        #[builder(into, default)]
        pub disk_types_not_alloweds: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The end of life date in RFC3339 format of the Image.
        #[builder(into, default)]
        pub end_of_life_date: pulumi_wasm_rust::Output<Option<String>>,
        /// The End User Licence Agreement for the Shared Image. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub eula: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Shared Image Gallery in which this Shared Image should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub gallery_name: pulumi_wasm_rust::Output<String>,
        /// Specifies if the Shared Image supports hibernation. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub hibernation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The generation of HyperV that the Virtual Machine used to create the Shared Image is based on. Possible values are `V1` and `V2`. Defaults to `V1`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub hyper_v_generation: pulumi_wasm_rust::Output<Option<String>>,
        /// An `identifier` block as defined below.
        #[builder(into)]
        pub identifier: pulumi_wasm_rust::Output<
            super::super::types::compute::SharedImageIdentifier,
        >,
        /// Specifies the supported Azure location where the Shared Image Gallery exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Maximum memory in GB recommended for the Image.
        #[builder(into, default)]
        pub max_recommended_memory_in_gb: pulumi_wasm_rust::Output<Option<i32>>,
        /// Maximum count of vCPUs recommended for the Image.
        #[builder(into, default)]
        pub max_recommended_vcpu_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Minimum memory in GB recommended for the Image.
        #[builder(into, default)]
        pub min_recommended_memory_in_gb: pulumi_wasm_rust::Output<Option<i32>>,
        /// Minimum count of vCPUs recommended for the Image.
        #[builder(into, default)]
        pub min_recommended_vcpu_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the name of the Shared Image. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of Operating System present in this Shared Image. Possible values are `Linux` and `Windows`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub os_type: pulumi_wasm_rust::Output<String>,
        /// The URI containing the Privacy Statement associated with this Shared Image. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub privacy_statement_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// A `purchase_plan` block as defined below.
        #[builder(into, default)]
        pub purchase_plan: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::SharedImagePurchasePlan>,
        >,
        /// The URI containing the Release Notes associated with this Shared Image.
        #[builder(into, default)]
        pub release_note_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which the Shared Image Gallery exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies that the Operating System used inside this Image has not been Generalized (for example, `sysprep` on Windows has not been run). Changing this forces a new resource to be created.
        ///
        /// !> **Note:** It's recommended to Generalize images where possible - Specialized Images reuse the same UUID internally within each Virtual Machine, which can have unintended side-effects.
        #[builder(into, default)]
        pub specialized: pulumi_wasm_rust::Output<Option<bool>>,
        /// A mapping of tags to assign to the Shared Image.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies if Trusted Launch has to be enabled for the Virtual Machine created from the Shared Image. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub trusted_launch_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies if supports creation of both Trusted Launch virtual machines and Gen2 virtual machines with standard security created from the Shared Image. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub trusted_launch_supported: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct SharedImageResult {
        /// Specifies if the Shared Image supports Accelerated Network. Changing this forces a new resource to be created.
        pub accelerated_network_support_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// CPU architecture supported by an OS. Possible values are `x64` and `Arm64`. Defaults to `x64`. Changing this forces a new resource to be created.
        pub architecture: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies if Confidential Virtual Machines enabled. It will enable all the features of trusted, with higher confidentiality features for isolate machines or encrypted data. Available for Gen2 machines. Changing this forces a new resource to be created.
        ///
        /// > **Note:**: Only one of `trusted_launch_supported`, `trusted_launch_enabled`, `confidential_vm_supported` and `confidential_vm_enabled` can be specified.
        pub confidential_vm_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies if supports creation of both Confidential virtual machines and Gen2 virtual machines with standard security from a compatible Gen2 OS disk VHD or Gen2 Managed image. Changing this forces a new resource to be created.
        pub confidential_vm_supported: pulumi_wasm_rust::Output<Option<bool>>,
        /// A description of this Shared Image.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies if the Shared Image supports NVMe disks. Changing this forces a new resource to be created.
        pub disk_controller_type_nvme_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more Disk Types not allowed for the Image. Possible values include `Standard_LRS` and `Premium_LRS`.
        pub disk_types_not_alloweds: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The end of life date in RFC3339 format of the Image.
        pub end_of_life_date: pulumi_wasm_rust::Output<Option<String>>,
        /// The End User Licence Agreement for the Shared Image. Changing this forces a new resource to be created.
        pub eula: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Shared Image Gallery in which this Shared Image should exist. Changing this forces a new resource to be created.
        pub gallery_name: pulumi_wasm_rust::Output<String>,
        /// Specifies if the Shared Image supports hibernation. Changing this forces a new resource to be created.
        pub hibernation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The generation of HyperV that the Virtual Machine used to create the Shared Image is based on. Possible values are `V1` and `V2`. Defaults to `V1`. Changing this forces a new resource to be created.
        pub hyper_v_generation: pulumi_wasm_rust::Output<Option<String>>,
        /// An `identifier` block as defined below.
        pub identifier: pulumi_wasm_rust::Output<
            super::super::types::compute::SharedImageIdentifier,
        >,
        /// Specifies the supported Azure location where the Shared Image Gallery exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Maximum memory in GB recommended for the Image.
        pub max_recommended_memory_in_gb: pulumi_wasm_rust::Output<Option<i32>>,
        /// Maximum count of vCPUs recommended for the Image.
        pub max_recommended_vcpu_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Minimum memory in GB recommended for the Image.
        pub min_recommended_memory_in_gb: pulumi_wasm_rust::Output<Option<i32>>,
        /// Minimum count of vCPUs recommended for the Image.
        pub min_recommended_vcpu_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the name of the Shared Image. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The type of Operating System present in this Shared Image. Possible values are `Linux` and `Windows`. Changing this forces a new resource to be created.
        pub os_type: pulumi_wasm_rust::Output<String>,
        /// The URI containing the Privacy Statement associated with this Shared Image. Changing this forces a new resource to be created.
        pub privacy_statement_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// A `purchase_plan` block as defined below.
        pub purchase_plan: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::SharedImagePurchasePlan>,
        >,
        /// The URI containing the Release Notes associated with this Shared Image.
        pub release_note_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which the Shared Image Gallery exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies that the Operating System used inside this Image has not been Generalized (for example, `sysprep` on Windows has not been run). Changing this forces a new resource to be created.
        ///
        /// !> **Note:** It's recommended to Generalize images where possible - Specialized Images reuse the same UUID internally within each Virtual Machine, which can have unintended side-effects.
        pub specialized: pulumi_wasm_rust::Output<Option<bool>>,
        /// A mapping of tags to assign to the Shared Image.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies if Trusted Launch has to be enabled for the Virtual Machine created from the Shared Image. Changing this forces a new resource to be created.
        pub trusted_launch_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies if supports creation of both Trusted Launch virtual machines and Gen2 virtual machines with standard security created from the Shared Image. Changing this forces a new resource to be created.
        pub trusted_launch_supported: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SharedImageArgs) -> SharedImageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let accelerated_network_support_enabled_binding = args
            .accelerated_network_support_enabled
            .get_inner();
        let architecture_binding = args.architecture.get_inner();
        let confidential_vm_enabled_binding = args.confidential_vm_enabled.get_inner();
        let confidential_vm_supported_binding = args
            .confidential_vm_supported
            .get_inner();
        let description_binding = args.description.get_inner();
        let disk_controller_type_nvme_enabled_binding = args
            .disk_controller_type_nvme_enabled
            .get_inner();
        let disk_types_not_alloweds_binding = args.disk_types_not_alloweds.get_inner();
        let end_of_life_date_binding = args.end_of_life_date.get_inner();
        let eula_binding = args.eula.get_inner();
        let gallery_name_binding = args.gallery_name.get_inner();
        let hibernation_enabled_binding = args.hibernation_enabled.get_inner();
        let hyper_v_generation_binding = args.hyper_v_generation.get_inner();
        let identifier_binding = args.identifier.get_inner();
        let location_binding = args.location.get_inner();
        let max_recommended_memory_in_gb_binding = args
            .max_recommended_memory_in_gb
            .get_inner();
        let max_recommended_vcpu_count_binding = args
            .max_recommended_vcpu_count
            .get_inner();
        let min_recommended_memory_in_gb_binding = args
            .min_recommended_memory_in_gb
            .get_inner();
        let min_recommended_vcpu_count_binding = args
            .min_recommended_vcpu_count
            .get_inner();
        let name_binding = args.name.get_inner();
        let os_type_binding = args.os_type.get_inner();
        let privacy_statement_uri_binding = args.privacy_statement_uri.get_inner();
        let purchase_plan_binding = args.purchase_plan.get_inner();
        let release_note_uri_binding = args.release_note_uri.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let specialized_binding = args.specialized.get_inner();
        let tags_binding = args.tags.get_inner();
        let trusted_launch_enabled_binding = args.trusted_launch_enabled.get_inner();
        let trusted_launch_supported_binding = args.trusted_launch_supported.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/sharedImage:SharedImage".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "acceleratedNetworkSupportEnabled".into(),
                },
                register_interface::ResultField {
                    name: "architecture".into(),
                },
                register_interface::ResultField {
                    name: "confidentialVmEnabled".into(),
                },
                register_interface::ResultField {
                    name: "confidentialVmSupported".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "diskControllerTypeNvmeEnabled".into(),
                },
                register_interface::ResultField {
                    name: "diskTypesNotAlloweds".into(),
                },
                register_interface::ResultField {
                    name: "endOfLifeDate".into(),
                },
                register_interface::ResultField {
                    name: "eula".into(),
                },
                register_interface::ResultField {
                    name: "galleryName".into(),
                },
                register_interface::ResultField {
                    name: "hibernationEnabled".into(),
                },
                register_interface::ResultField {
                    name: "hyperVGeneration".into(),
                },
                register_interface::ResultField {
                    name: "identifier".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "maxRecommendedMemoryInGb".into(),
                },
                register_interface::ResultField {
                    name: "maxRecommendedVcpuCount".into(),
                },
                register_interface::ResultField {
                    name: "minRecommendedMemoryInGb".into(),
                },
                register_interface::ResultField {
                    name: "minRecommendedVcpuCount".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "osType".into(),
                },
                register_interface::ResultField {
                    name: "privacyStatementUri".into(),
                },
                register_interface::ResultField {
                    name: "purchasePlan".into(),
                },
                register_interface::ResultField {
                    name: "releaseNoteUri".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "specialized".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "trustedLaunchEnabled".into(),
                },
                register_interface::ResultField {
                    name: "trustedLaunchSupported".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SharedImageResult {
            accelerated_network_support_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acceleratedNetworkSupportEnabled").unwrap(),
            ),
            architecture: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("architecture").unwrap(),
            ),
            confidential_vm_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("confidentialVmEnabled").unwrap(),
            ),
            confidential_vm_supported: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("confidentialVmSupported").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disk_controller_type_nvme_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskControllerTypeNvmeEnabled").unwrap(),
            ),
            disk_types_not_alloweds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskTypesNotAlloweds").unwrap(),
            ),
            end_of_life_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endOfLifeDate").unwrap(),
            ),
            eula: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eula").unwrap(),
            ),
            gallery_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("galleryName").unwrap(),
            ),
            hibernation_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hibernationEnabled").unwrap(),
            ),
            hyper_v_generation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hyperVGeneration").unwrap(),
            ),
            identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identifier").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            max_recommended_memory_in_gb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxRecommendedMemoryInGb").unwrap(),
            ),
            max_recommended_vcpu_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxRecommendedVcpuCount").unwrap(),
            ),
            min_recommended_memory_in_gb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minRecommendedMemoryInGb").unwrap(),
            ),
            min_recommended_vcpu_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minRecommendedVcpuCount").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            os_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osType").unwrap(),
            ),
            privacy_statement_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privacyStatementUri").unwrap(),
            ),
            purchase_plan: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("purchasePlan").unwrap(),
            ),
            release_note_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("releaseNoteUri").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            specialized: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("specialized").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            trusted_launch_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustedLaunchEnabled").unwrap(),
            ),
            trusted_launch_supported: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustedLaunchSupported").unwrap(),
            ),
        }
    }
}
