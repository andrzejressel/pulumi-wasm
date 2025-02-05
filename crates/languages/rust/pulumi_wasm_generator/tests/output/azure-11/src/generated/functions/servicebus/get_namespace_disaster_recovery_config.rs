pub mod get_namespace_disaster_recovery_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNamespaceDisasterRecoveryConfigArgs {
        #[builder(into, default)]
        pub alias_authorization_rule_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub namespace_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub namespace_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetNamespaceDisasterRecoveryConfigArgs,
    ) -> GetNamespaceDisasterRecoveryConfigResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alias_authorization_rule_id_binding = args
            .alias_authorization_rule_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let namespace_id_binding = args.namespace_id.get_output(context).get_inner();
        let namespace_name_binding = args.namespace_name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetNamespaceDisasterRecoveryConfigResult {
            alias_authorization_rule_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("aliasAuthorizationRuleId"),
            ),
            default_primary_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultPrimaryKey"),
            ),
            default_secondary_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultSecondaryKey"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            namespace_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("namespaceId"),
            ),
            namespace_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("namespaceName"),
            ),
            partner_namespace_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("partnerNamespaceId"),
            ),
            primary_connection_string_alias: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryConnectionStringAlias"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            secondary_connection_string_alias: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryConnectionStringAlias"),
            ),
        }
    }
}
