/// Manages a Dev Center Gallery.
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
///   test:
///     type: azure:devcenter:DevCenter
///     properties:
///       name: example-devcenter
///       resourceGroupName: ${testAzurermResourceGroup.name}
///       location: ${testAzurermResourceGroup.location}
///       identity:
///         type: UserAssigned
///         identityIds:
///           - ${testUserAssignedIdentity.id}
///   testUserAssignedIdentity:
///     type: azure:authorization:UserAssignedIdentity
///     name: test
///     properties:
///       name: example-uai
///       location: ${testAzurermResourceGroup.location}
///       resourceGroupName: ${testAzurermResourceGroup.name}
///   exampleSharedImageGallery:
///     type: azure:compute:SharedImageGallery
///     name: example
///     properties:
///       name: example-image-gallery
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleGallery:
///     type: azure:devcenter:Gallery
///     name: example
///     properties:
///       devCenterId: ${exampleAzurermDevCenter.id}
///       sharedGalleryId: ${exampleSharedImageGallery.id}
///       name: example
/// ```
///
/// ## Import
///
/// An existing Dev Center Gallery can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:devcenter/gallery:Gallery example /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.DevCenter/devCenters/{devCenterName}/galleries/{galleryName}
/// ```
///
/// * Where `{subscriptionId}` is the ID of the Azure Subscription where the Dev Center Gallery exists. For example `12345678-1234-9876-4563-123456789012`.
///
/// * Where `{resourceGroupName}` is the name of Resource Group where this Dev Center Gallery exists. For example `example-resource-group`.
///
/// * Where `{devCenterName}` is the name of the Dev Center. For example `devCenterValue`.
///
/// * Where `{galleryName}` is the name of the Gallery. For example `galleryValue`.
///
pub mod gallery {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GalleryArgs {
        /// Specifies the ID of the Dev Center within which this Dev Center Gallery should exist. Changing this forces a new Dev Center Gallery to be created.
        #[builder(into)]
        pub dev_center_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of this Dev Center Gallery. Changing this forces a new Dev Center Gallery to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Shared Gallery which should be connected to the Dev Center Gallery. Changing this forces a new Dev Center Gallery to be created.
        #[builder(into)]
        pub shared_gallery_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GalleryResult {
        /// Specifies the ID of the Dev Center within which this Dev Center Gallery should exist. Changing this forces a new Dev Center Gallery to be created.
        pub dev_center_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of this Dev Center Gallery. Changing this forces a new Dev Center Gallery to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Shared Gallery which should be connected to the Dev Center Gallery. Changing this forces a new Dev Center Gallery to be created.
        pub shared_gallery_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: GalleryArgs) -> GalleryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dev_center_id_binding = args.dev_center_id.get_inner();
        let name_binding = args.name.get_inner();
        let shared_gallery_id_binding = args.shared_gallery_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:devcenter/gallery:Gallery".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "devCenterId".into(),
                    value: &dev_center_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "sharedGalleryId".into(),
                    value: &shared_gallery_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "devCenterId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "sharedGalleryId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GalleryResult {
            dev_center_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("devCenterId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            shared_gallery_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sharedGalleryId").unwrap(),
            ),
        }
    }
}
