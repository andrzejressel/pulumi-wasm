pub mod get_groups {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGroupsArgs {
        /// Identity Store ID associated with the Single Sign-On (SSO) Instance.
        #[builder(into)]
        pub identity_store_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetGroupsResult {
        /// List of Identity Store Groups
        pub groups: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::identitystore::GetGroupsGroup>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub identity_store_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetGroupsArgs) -> GetGroupsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let identity_store_id_binding = args.identity_store_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:identitystore/getGroups:getGroups".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "identityStoreId".into(),
                    value: &identity_store_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "groups".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identityStoreId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetGroupsResult {
            groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groups").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identity_store_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityStoreId").unwrap(),
            ),
        }
    }
}
