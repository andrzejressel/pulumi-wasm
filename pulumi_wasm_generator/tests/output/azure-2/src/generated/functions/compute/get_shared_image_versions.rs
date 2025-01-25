pub mod get_shared_image_versions {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSharedImageVersionsArgs {
        /// The name of the Shared Image in which the Shared Image exists.
        #[builder(into)]
        pub gallery_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Shared Image in which this Version exists.
        #[builder(into)]
        pub image_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Shared Image Gallery exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags to filter the list of images against.
        #[builder(into, default)]
        pub tags_filter: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSharedImageVersionsResult {
        pub gallery_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub image_name: pulumi_wasm_rust::Output<String>,
        /// An `images` block as defined below:
        pub images: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetSharedImageVersionsImage>,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub tags_filter: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetSharedImageVersionsArgs,
    ) -> GetSharedImageVersionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let gallery_name_binding = args.gallery_name.get_output(context).get_inner();
        let image_name_binding = args.image_name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_filter_binding = args.tags_filter.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:compute/getSharedImageVersions:getSharedImageVersions".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "galleryName".into(),
                    value: &gallery_name_binding,
                },
                register_interface::ObjectField {
                    name: "imageName".into(),
                    value: &image_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tagsFilter".into(),
                    value: &tags_filter_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "galleryName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "imageName".into(),
                },
                register_interface::ResultField {
                    name: "images".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tagsFilter".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSharedImageVersionsResult {
            gallery_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("galleryName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            image_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageName").unwrap(),
            ),
            images: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("images").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags_filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsFilter").unwrap(),
            ),
        }
    }
}
