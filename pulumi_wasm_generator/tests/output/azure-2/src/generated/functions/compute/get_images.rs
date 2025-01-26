pub mod get_images {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImagesArgs {
        /// The name of the Resource Group in which the Image exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags to filter the list of images against.
        #[builder(into, default)]
        pub tags_filter: pulumi_wasm_rust::InputOrOutput<
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetImagesArgs,
    ) -> GetImagesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_filter_binding = args.tags_filter.get_output(context).get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetImagesResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            images: pulumi_wasm_rust::__private::into_domain(o.extract_field("images")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags_filter: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsFilter"),
            ),
        }
    }
}
