pub mod get_tag_values {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTagValuesArgs {
        /// The resource name of the parent tagKey in format `tagKey/{name}`.
        #[builder(into)]
        pub parent: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTagValuesResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The resource name of the new TagValue's parent TagKey. Must be of the form tagKeys/{tag_key_id}.
        pub parent: pulumi_wasm_rust::Output<String>,
        pub values: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::tags::GetTagValuesValue>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetTagValuesArgs,
    ) -> GetTagValuesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let parent_binding = args.parent.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:tags/getTagValues:getTagValues".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetTagValuesResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            parent: pulumi_wasm_rust::__private::into_domain(o.extract_field("parent")),
            values: pulumi_wasm_rust::__private::into_domain(o.extract_field("values")),
        }
    }
}
