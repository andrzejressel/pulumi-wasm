pub mod get_images {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImagesArgs {
        /// The name of the Resource Group in which the Image exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to filter the list of images against.
        #[builder(into, default)]
        pub tags_filter: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetImagesResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// One or more `images` blocks as defined below:
        pub images: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetImagesImage>,
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
    pub fn invoke(args: GetImagesArgs) -> GetImagesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_filter_binding = args.tags_filter.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:compute/getImages:getImages".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
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
                    name: "id".into(),
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
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetImagesResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
