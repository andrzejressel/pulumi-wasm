pub mod get_namespace_disaster_recovery_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNamespaceDisasterRecoveryConfigArgs {
        #[builder(into, default)]
        pub alias_authorization_rule_id: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub namespace_id: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub namespace_name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub resource_group_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetNamespaceDisasterRecoveryConfigResult {
        pub alias_authorization_rule_id: pulumi_wasm_rust::Output<Option<String>>,
        pub default_primary_key: pulumi_wasm_rust::Output<String>,
        pub default_secondary_key: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub namespace_id: pulumi_wasm_rust::Output<Option<String>>,
        pub namespace_name: pulumi_wasm_rust::Output<Option<String>>,
        pub partner_namespace_id: pulumi_wasm_rust::Output<String>,
        pub primary_connection_string_alias: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<Option<String>>,
        pub secondary_connection_string_alias: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetNamespaceDisasterRecoveryConfigArgs,
    ) -> GetNamespaceDisasterRecoveryConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alias_authorization_rule_id_binding = args
            .alias_authorization_rule_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let namespace_id_binding = args.namespace_id.get_inner();
        let namespace_name_binding = args.namespace_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:servicebus/getNamespaceDisasterRecoveryConfig:getNamespaceDisasterRecoveryConfig"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "aliasAuthorizationRuleId".into(),
                    value: &alias_authorization_rule_id_binding,
                },
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
                    name: "aliasAuthorizationRuleId".into(),
                },
                register_interface::ResultField {
                    name: "defaultPrimaryKey".into(),
                },
                register_interface::ResultField {
                    name: "defaultSecondaryKey".into(),
                },
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
                    name: "partnerNamespaceId".into(),
                },
                register_interface::ResultField {
                    name: "primaryConnectionStringAlias".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "secondaryConnectionStringAlias".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNamespaceDisasterRecoveryConfigResult {
            alias_authorization_rule_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aliasAuthorizationRuleId").unwrap(),
            ),
            default_primary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultPrimaryKey").unwrap(),
            ),
            default_secondary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSecondaryKey").unwrap(),
            ),
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
            partner_namespace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partnerNamespaceId").unwrap(),
            ),
            primary_connection_string_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryConnectionStringAlias").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            secondary_connection_string_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryConnectionStringAlias").unwrap(),
            ),
        }
    }
}
