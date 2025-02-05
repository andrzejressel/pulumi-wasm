pub mod get_namespace_authorization_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNamespaceAuthorizationRuleArgs {
        /// The name of the EventHub Authorization Rule resource.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the EventHub Namespace.
        #[builder(into)]
        pub namespace_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the resource group in which the EventHub Namespace exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNamespaceAuthorizationRuleResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Does this Authorization Rule have permissions to Listen to the Event Hub?
        pub listen: pulumi_wasm_rust::Output<bool>,
        /// Does this Authorization Rule have permissions to Manage to the Event Hub?
        pub manage: pulumi_wasm_rust::Output<bool>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub namespace_name: pulumi_wasm_rust::Output<String>,
        /// The Primary Connection String for the Event Hubs authorization Rule.
        pub primary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The alias of the Primary Connection String for the Event Hubs authorization Rule.
        pub primary_connection_string_alias: pulumi_wasm_rust::Output<String>,
        /// The Primary Key for the Event Hubs authorization Rule.
        pub primary_key: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Secondary Connection String for the Event Hubs authorization Rule.
        pub secondary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The alias of the Secondary Connection String for the Event Hubs authorization Rule.
        pub secondary_connection_string_alias: pulumi_wasm_rust::Output<String>,
        /// The Secondary Key for the Event Hubs authorization Rule.
        pub secondary_key: pulumi_wasm_rust::Output<String>,
        /// Does this Authorization Rule have permissions to Send to the Event Hub?
        pub send: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetNamespaceAuthorizationRuleArgs,
    ) -> GetNamespaceAuthorizationRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let namespace_name_binding = args.namespace_name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:eventhub/getNamespaceAuthorizationRule:getNamespaceAuthorizationRule"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
        GetNamespaceAuthorizationRuleResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            listen: pulumi_wasm_rust::__private::into_domain(o.extract_field("listen")),
            manage: pulumi_wasm_rust::__private::into_domain(o.extract_field("manage")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            namespace_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("namespaceName"),
            ),
            primary_connection_string: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryConnectionString"),
            ),
            primary_connection_string_alias: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryConnectionStringAlias"),
            ),
            primary_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryKey"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            secondary_connection_string: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryConnectionString"),
            ),
            secondary_connection_string_alias: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryConnectionStringAlias"),
            ),
            secondary_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryKey"),
            ),
            send: pulumi_wasm_rust::__private::into_domain(o.extract_field("send")),
        }
    }
}
