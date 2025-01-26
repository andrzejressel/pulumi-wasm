pub mod get_tag_value {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTagValueArgs {
        /// The resource name of the parent tagKey in format `tagKey/{name}`.
        #[builder(into)]
        pub parent: pulumi_wasm_rust::InputOrOutput<String>,
        /// The tag value's short_name.
        #[builder(into)]
        pub short_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTagValueResult {
        /// Creation time.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        /// an identifier for the resource with format `tagValues/{{name}}`
        pub id: pulumi_wasm_rust::Output<String>,
        /// The generated numeric id for the TagValue.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Namespaced name of the TagValue.
        pub namespaced_name: pulumi_wasm_rust::Output<String>,
        pub parent: pulumi_wasm_rust::Output<String>,
        pub short_name: pulumi_wasm_rust::Output<String>,
        /// Update time.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetTagValueArgs,
    ) -> GetTagValueResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let parent_binding = args.parent.get_output(context).get_inner();
        let short_name_binding = args.short_name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:tags/getTagValue:getTagValue".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "shortName".into(),
                    value: &short_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetTagValueResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            namespaced_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("namespacedName"),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(o.extract_field("parent")),
            short_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("shortName"),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
