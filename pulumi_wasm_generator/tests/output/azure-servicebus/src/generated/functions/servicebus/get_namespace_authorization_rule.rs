pub mod get_namespace_authorization_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNamespaceAuthorizationRuleArgs {
        /// Specifies the name of the ServiceBus Namespace Authorization Rule.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the ServiceBus Namespace where the Service Bus Namespace Authorization Rule exists.
        #[builder(into, default)]
        pub namespace_id: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub namespace_name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub resource_group_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetNamespaceAuthorizationRuleResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub namespace_id: pulumi_wasm_rust::Output<Option<String>>,
        pub namespace_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The primary connection string for the authorization rule.
        pub primary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The alias Primary Connection String for the ServiceBus Namespace, if the namespace is Geo DR paired.
        pub primary_connection_string_alias: pulumi_wasm_rust::Output<String>,
        /// The primary access key for the authorization rule.
        pub primary_key: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The secondary connection string for the authorization rule.
        pub secondary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The alias Secondary Connection String for the ServiceBus Namespace
        pub secondary_connection_string_alias: pulumi_wasm_rust::Output<String>,
        /// The secondary access key for the authorization rule.
        pub secondary_key: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetNamespaceAuthorizationRuleArgs,
    ) -> GetNamespaceAuthorizationRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let namespace_id_binding = args.namespace_id.get_inner();
        let namespace_name_binding = args.namespace_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:servicebus/getNamespaceAuthorizationRule:getNamespaceAuthorizationRule"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namespaceId".into(),
                    value: &namespace_id_binding,
                },
                register_interface::ObjectField {
                    name: "namespaceName".into(),
                    value: &namespace_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namespaceId".into(),
                },
                register_interface::ResultField {
                    name: "namespaceName".into(),
                },
                register_interface::ResultField {
                    name: "primaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "primaryConnectionStringAlias".into(),
                },
                register_interface::ResultField {
                    name: "primaryKey".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "secondaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "secondaryConnectionStringAlias".into(),
                },
                register_interface::ResultField {
                    name: "secondaryKey".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNamespaceAuthorizationRuleResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            namespace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespaceId").unwrap(),
            ),
            namespace_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespaceName").unwrap(),
            ),
            primary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryConnectionString").unwrap(),
            ),
            primary_connection_string_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryConnectionStringAlias").unwrap(),
            ),
            primary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryKey").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            secondary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryConnectionString").unwrap(),
            ),
            secondary_connection_string_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryConnectionStringAlias").unwrap(),
            ),
            secondary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryKey").unwrap(),
            ),
        }
    }
}
