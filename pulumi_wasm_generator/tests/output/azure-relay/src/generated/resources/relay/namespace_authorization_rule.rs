/// Manages an Azure Relay Namespace Authorization Rule.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleNamespace:
///     type: azure:relay:Namespace
///     name: example
///     properties:
///       name: example-relay
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: Standard
///       tags:
///         source: terraform
///   exampleNamespaceAuthorizationRule:
///     type: azure:relay:NamespaceAuthorizationRule
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       namespaceName: ${exampleNamespace.name}
///       listen: true
///       send: true
///       manage: false
/// ```
///
/// ## Import
///
/// Azure Relay Namespace Authorization Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:relay/namespaceAuthorizationRule:NamespaceAuthorizationRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Relay/namespaces/namespace1/authorizationRules/rule1
/// ```
///
pub mod namespace_authorization_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NamespaceAuthorizationRuleArgs {
        /// Grants listen access to this Authorization Rule. Defaults to `false`.
        #[builder(into, default)]
        pub listen: pulumi_wasm_rust::Output<Option<bool>>,
        /// Grants manage access to this Authorization Rule. When this property is `true` - both `listen` and `send` must be set to `true` too. Defaults to `false`.
        #[builder(into, default)]
        pub manage: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Azure Relay Namespace Authorization Rule. Changing this forces a new Azure Relay Namespace Authorization Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the Azure Relay Namespace for which this Azure Relay Namespace Authorization Rule will be created. Changing this forces a new Azure Relay Namespace Authorization Rule to be created.
        #[builder(into)]
        pub namespace_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Azure Relay Namespace Authorization Rule should exist. Changing this forces a new Azure Relay Namespace Authorization Rule to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Grants send access to this Authorization Rule. Defaults to `false`.
        #[builder(into, default)]
        pub send: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct NamespaceAuthorizationRuleResult {
        /// Grants listen access to this Authorization Rule. Defaults to `false`.
        pub listen: pulumi_wasm_rust::Output<Option<bool>>,
        /// Grants manage access to this Authorization Rule. When this property is `true` - both `listen` and `send` must be set to `true` too. Defaults to `false`.
        pub manage: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Azure Relay Namespace Authorization Rule. Changing this forces a new Azure Relay Namespace Authorization Rule to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Name of the Azure Relay Namespace for which this Azure Relay Namespace Authorization Rule will be created. Changing this forces a new Azure Relay Namespace Authorization Rule to be created.
        pub namespace_name: pulumi_wasm_rust::Output<String>,
        /// The Primary Connection String for the Azure Relay Namespace Authorization Rule.
        pub primary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The Primary Key for the Azure Relay Namespace Authorization Rule.
        pub primary_key: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Azure Relay Namespace Authorization Rule should exist. Changing this forces a new Azure Relay Namespace Authorization Rule to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Secondary Connection String for the Azure Relay Namespace Authorization Rule.
        pub secondary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The Secondary Key for the Azure Relay Namespace Authorization Rule.
        pub secondary_key: pulumi_wasm_rust::Output<String>,
        /// Grants send access to this Authorization Rule. Defaults to `false`.
        pub send: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NamespaceAuthorizationRuleArgs,
    ) -> NamespaceAuthorizationRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let listen_binding = args.listen.get_inner();
        let manage_binding = args.manage.get_inner();
        let name_binding = args.name.get_inner();
        let namespace_name_binding = args.namespace_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let send_binding = args.send.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:relay/namespaceAuthorizationRule:NamespaceAuthorizationRule"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "listen".into(),
                    value: &listen_binding,
                },
                register_interface::ObjectField {
                    name: "manage".into(),
                    value: &manage_binding,
                },
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
                register_interface::ObjectField {
                    name: "send".into(),
                    value: &send_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "listen".into(),
                },
                register_interface::ResultField {
                    name: "manage".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namespaceName".into(),
                },
                register_interface::ResultField {
                    name: "primaryConnectionString".into(),
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
                    name: "secondaryKey".into(),
                },
                register_interface::ResultField {
                    name: "send".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NamespaceAuthorizationRuleResult {
            listen: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("listen").unwrap(),
            ),
            manage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("manage").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            namespace_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespaceName").unwrap(),
            ),
            primary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryConnectionString").unwrap(),
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
            secondary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryKey").unwrap(),
            ),
            send: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("send").unwrap(),
            ),
        }
    }
}
