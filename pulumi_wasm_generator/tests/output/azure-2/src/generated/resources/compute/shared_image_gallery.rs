/// Manages a Shared Image Gallery.
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
/// ```
///
/// ## Import
///
/// Shared Image Galleries can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/sharedImageGallery:SharedImageGallery gallery1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/galleries/gallery1
/// ```
///
pub mod shared_image_gallery {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SharedImageGalleryArgs {
        /// A description for this Shared Image Gallery.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Shared Image Gallery. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Shared Image Gallery. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A `sharing` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub sharing: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::SharedImageGallerySharing>,
        >,
        /// A mapping of tags to assign to the Shared Image Gallery.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SharedImageGalleryResult {
        /// A description for this Shared Image Gallery.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Shared Image Gallery. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the Shared Image Gallery. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `sharing` block as defined below. Changing this forces a new resource to be created.
        pub sharing: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::SharedImageGallerySharing>,
        >,
        /// A mapping of tags to assign to the Shared Image Gallery.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Unique Name for this Shared Image Gallery.
        pub unique_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SharedImageGalleryArgs,
    ) -> SharedImageGalleryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sharing_binding = args.sharing.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/sharedImageGallery:SharedImageGallery".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
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
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sharing".into(),
                    value: &sharing_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SharedImageGalleryResult {
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sharing: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sharing"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            unique_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("uniqueName"),
            ),
        }
    }
}
