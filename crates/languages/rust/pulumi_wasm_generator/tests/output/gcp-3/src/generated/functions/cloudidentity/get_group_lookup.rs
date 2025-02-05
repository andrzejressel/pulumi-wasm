pub mod get_group_lookup {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGroupLookupArgs {
        /// The EntityKey of the Group to lookup. A unique identifier for an entity in the Cloud Identity Groups API.
        /// An entity can represent either a group with an optional namespace or a user without a namespace.
        /// The combination of id and namespace must be unique; however, the same id can be used with different namespaces. Structure is documented below.
        #[builder(into)]
        pub group_key: pulumi_wasm_rust::InputOrOutput<
            super::super::super::types::cloudidentity::GetGroupLookupGroupKey,
        >,
    }
    #[allow(dead_code)]
    pub struct GetGroupLookupResult {
        pub group_key: pulumi_wasm_rust::Output<
            super::super::super::types::cloudidentity::GetGroupLookupGroupKey,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Resource name of the Group in the format: groups/{group_id}, where `group_id` is the unique ID assigned to the Group.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetGroupLookupArgs,
    ) -> GetGroupLookupResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let group_key_binding = args.group_key.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:cloudidentity/getGroupLookup:getGroupLookup".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "groupKey".into(),
                    value: &group_key_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetGroupLookupResult {
            group_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("groupKey"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
