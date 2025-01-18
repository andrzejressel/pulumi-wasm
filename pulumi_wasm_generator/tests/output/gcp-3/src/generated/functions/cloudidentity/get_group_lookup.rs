pub mod get_group_lookup {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGroupLookupArgs {
        /// The EntityKey of the Group to lookup. A unique identifier for an entity in the Cloud Identity Groups API.
        /// An entity can represent either a group with an optional namespace or a user without a namespace.
        /// The combination of id and namespace must be unique; however, the same id can be used with different namespaces. Structure is documented below.
        #[builder(into)]
        pub group_key: pulumi_wasm_rust::Output<
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
    pub fn invoke(args: GetGroupLookupArgs) -> GetGroupLookupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let group_key_binding = args.group_key.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:cloudidentity/getGroupLookup:getGroupLookup".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "groupKey".into(),
                    value: &group_key_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "groupKey".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetGroupLookupResult {
            group_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupKey").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
