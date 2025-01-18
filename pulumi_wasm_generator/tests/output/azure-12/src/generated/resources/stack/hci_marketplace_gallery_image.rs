/// Manages an Azure Stack HCI Marketplace Gallery Image.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: examples
///       location: West Europe
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       scope: ${exampleResourceGroup.id}
///       roleDefinitionName: Azure Connected Machine Resource Manager
///       principalId: ${hciRp.objectId}
///   exampleHciMarketplaceGalleryImage:
///     type: azure:stack:HciMarketplaceGalleryImage
///     name: example
///     properties:
///       name: example-mgi
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       customLocationId: /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.ExtendedLocation/customLocations/cl1
///       hypervGeneration: V2
///       osType: Windows
///       version: 20348.2655.240905
///       identifier:
///         publisher: MicrosoftWindowsServer
///         offer: WindowsServer
///         sku: 2022-datacenter-azure-edition-core
///       tags:
///         foo: bar
///         env: example
/// variables:
///   example:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
///   # service principal of 'Microsoft.AzureStackHCI Resource Provider'
///   hciRp:
///     fn::invoke:
///       function: azuread:getServicePrincipal
///       arguments:
///         clientId: 1412d89f-b8a8-4111-b4fd-e82905cbd85d
/// ```
///
/// ## Import
///
/// Azure Stack HCI Marketplace Gallery Images can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:stack/hciMarketplaceGalleryImage:HciMarketplaceGalleryImage example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.AzureStackHCI/marketplaceGalleryImages/image1
/// ```
///
pub mod hci_marketplace_gallery_image {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HciMarketplaceGalleryImageArgs {
        /// The ID of the Custom Location where the Azure Stack HCI Marketplace Gallery Image should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub custom_location_id: pulumi_wasm_rust::Output<String>,
        /// The hypervisor generation of the Azure Stack HCI Marketplace Gallery Image. Possible values are `V1` and `V2`. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        #[builder(into)]
        pub hyperv_generation: pulumi_wasm_rust::Output<String>,
        /// An `identifier` block as defined below. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        #[builder(into)]
        pub identifier: pulumi_wasm_rust::Output<
            super::super::types::stack::HciMarketplaceGalleryImageIdentifier,
        >,
        /// The Azure Region where the Azure Stack HCI Marketplace Gallery Image should exist. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Azure Stack HCI Marketplace Gallery Image. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Operating System type of the Azure Stack HCI Marketplace Gallery Image. Possible values are `Windows` and `Linux`. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        #[builder(into)]
        pub os_type: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Azure Stack HCI Marketplace Gallery Image should exist. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Azure Stack HCI Storage Path used for this Marketplace Gallery Image. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        #[builder(into, default)]
        pub storage_path_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Azure Stack HCI Marketplace Gallery Image.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The version of the Azure Stack HCI Marketplace Gallery Image. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        #[builder(into)]
        pub version: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct HciMarketplaceGalleryImageResult {
        /// The ID of the Custom Location where the Azure Stack HCI Marketplace Gallery Image should exist. Changing this forces a new resource to be created.
        pub custom_location_id: pulumi_wasm_rust::Output<String>,
        /// The hypervisor generation of the Azure Stack HCI Marketplace Gallery Image. Possible values are `V1` and `V2`. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        pub hyperv_generation: pulumi_wasm_rust::Output<String>,
        /// An `identifier` block as defined below. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        pub identifier: pulumi_wasm_rust::Output<
            super::super::types::stack::HciMarketplaceGalleryImageIdentifier,
        >,
        /// The Azure Region where the Azure Stack HCI Marketplace Gallery Image should exist. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Azure Stack HCI Marketplace Gallery Image. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Operating System type of the Azure Stack HCI Marketplace Gallery Image. Possible values are `Windows` and `Linux`. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        pub os_type: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Azure Stack HCI Marketplace Gallery Image should exist. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Azure Stack HCI Storage Path used for this Marketplace Gallery Image. Changing this forces a new Azure Stack HCI Virtual Hard Disk to be created.
        pub storage_path_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Azure Stack HCI Marketplace Gallery Image.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The version of the Azure Stack HCI Marketplace Gallery Image. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: HciMarketplaceGalleryImageArgs,
    ) -> HciMarketplaceGalleryImageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let custom_location_id_binding = args.custom_location_id.get_inner();
        let hyperv_generation_binding = args.hyperv_generation.get_inner();
        let identifier_binding = args.identifier.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let os_type_binding = args.os_type.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let storage_path_id_binding = args.storage_path_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:stack/hciMarketplaceGalleryImage:HciMarketplaceGalleryImage"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customLocationId".into(),
                    value: &custom_location_id_binding,
                },
                register_interface::ObjectField {
                    name: "hypervGeneration".into(),
                    value: &hyperv_generation_binding,
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "osType".into(),
                    value: &os_type_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "storagePathId".into(),
                    value: &storage_path_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "customLocationId".into(),
                },
                register_interface::ResultField {
                    name: "hypervGeneration".into(),
                },
                register_interface::ResultField {
                    name: "identifier".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "osType".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "storagePathId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HciMarketplaceGalleryImageResult {
            custom_location_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customLocationId").unwrap(),
            ),
            hyperv_generation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hypervGeneration").unwrap(),
            ),
            identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identifier").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            os_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osType").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            storage_path_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storagePathId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
