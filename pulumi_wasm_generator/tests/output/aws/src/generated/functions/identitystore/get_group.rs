pub mod get_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGroupArgs {
        /// A unique identifier for the group that is not the primary identifier. Conflicts with `group_id` and `filter`. Detailed below.
        #[builder(into, default)]
        pub alternate_identifier: pulumi_wasm_rust::Output<
            Option<
                super::super::super::types::identitystore::GetGroupAlternateIdentifier,
            >,
        >,
        /// Configuration block for filtering by a unique attribute of the group. Detailed below.
        #[builder(into, default)]
        pub filter: pulumi_wasm_rust::Output<
            Option<super::super::super::types::identitystore::GetGroupFilter>,
        >,
        /// The identifier for a group in the Identity Store.
        ///
        /// > Exactly one of the above arguments must be provided. Passing both `filter` and `group_id` is allowed for backwards compatibility.
        #[builder(into, default)]
        pub group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Identity Store ID associated with the Single Sign-On Instance.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub identity_store_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetGroupResult {
        pub alternate_identifier: pulumi_wasm_rust::Output<
            Option<
                super::super::super::types::identitystore::GetGroupAlternateIdentifier,
            >,
        >,
        /// Description of the specified group.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Group's display name value.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// List of identifiers issued to this resource by an external identity provider.
        pub external_ids: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::identitystore::GetGroupExternalId>,
        >,
        pub filter: pulumi_wasm_rust::Output<
            Option<super::super::super::types::identitystore::GetGroupFilter>,
        >,
        pub group_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub identity_store_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetGroupArgs) -> GetGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alternate_identifier_binding = args.alternate_identifier.get_inner();
        let filter_binding = args.filter.get_inner();
        let group_id_binding = args.group_id.get_inner();
        let identity_store_id_binding = args.identity_store_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:identitystore/getGroup:getGroup".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alternateIdentifier".into(),
                    value: &alternate_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "groupId".into(),
                    value: &group_id_binding,
                },
                register_interface::ObjectField {
                    name: "identityStoreId".into(),
                    value: &identity_store_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "alternateIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "externalIds".into(),
                },
                register_interface::ResultField {
                    name: "filter".into(),
                },
                register_interface::ResultField {
                    name: "groupId".into(),
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
        GetGroupResult {
            alternate_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alternateIdentifier").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            external_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("externalIds").unwrap(),
            ),
            filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filter").unwrap(),
            ),
            group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identity_store_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityStoreId").unwrap(),
            ),
        }
    }
}