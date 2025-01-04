/// Manages a Disaster Recovery Config for a Service Bus Namespace.
///
/// > **NOTE:** Disaster Recovery Config is a Premium SKU only capability.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: servicebus-replication
///       location: West Europe
///   primary:
///     type: azure:servicebus:Namespace
///     properties:
///       name: servicebus-primary
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Premium
///       capacity: '1'
///   secondary:
///     type: azure:servicebus:Namespace
///     properties:
///       name: servicebus-secondary
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Premium
///       capacity: '1'
///   exampleNamespaceAuthorizationRule:
///     type: azure:servicebus:NamespaceAuthorizationRule
///     name: example
///     properties:
///       name: examplerule
///       namespaceId: ${exampleAzurermServicebusNamespace.id}
///       listen: true
///       send: true
///       manage: false
///   exampleNamespaceDisasterRecoveryConfig:
///     type: azure:servicebus:NamespaceDisasterRecoveryConfig
///     name: example
///     properties:
///       name: servicebus-alias-name
///       primaryNamespaceId: ${primary.id}
///       partnerNamespaceId: ${secondary.id}
///       aliasAuthorizationRuleId: ${exampleNamespaceAuthorizationRule.id}
/// ```
///
/// ## Import
///
/// Service Bus DR configs can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:servicebus/namespaceDisasterRecoveryConfig:NamespaceDisasterRecoveryConfig config1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ServiceBus/namespaces/namespace1/disasterRecoveryConfigs/config1
/// ```
///
pub mod namespace_disaster_recovery_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NamespaceDisasterRecoveryConfigArgs {
        /// The Shared access policies used to access the connection string for the alias.
        #[builder(into, default)]
        pub alias_authorization_rule_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Disaster Recovery Config. This is the alias DNS name that will be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Service Bus Namespace to replicate to.
        #[builder(into)]
        pub partner_namespace_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the primary Service Bus Namespace to replicate. Changing this forces a new resource to be created.
        #[builder(into)]
        pub primary_namespace_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct NamespaceDisasterRecoveryConfigResult {
        /// The Shared access policies used to access the connection string for the alias.
        pub alias_authorization_rule_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The primary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_primary_key: pulumi_wasm_rust::Output<String>,
        /// The secondary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_key: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Disaster Recovery Config. This is the alias DNS name that will be created. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Service Bus Namespace to replicate to.
        pub partner_namespace_id: pulumi_wasm_rust::Output<String>,
        /// The alias Primary Connection String for the ServiceBus Namespace.
        pub primary_connection_string_alias: pulumi_wasm_rust::Output<String>,
        /// The ID of the primary Service Bus Namespace to replicate. Changing this forces a new resource to be created.
        pub primary_namespace_id: pulumi_wasm_rust::Output<String>,
        /// The alias Secondary Connection String for the ServiceBus Namespace
        pub secondary_connection_string_alias: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NamespaceDisasterRecoveryConfigArgs,
    ) -> NamespaceDisasterRecoveryConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alias_authorization_rule_id_binding = args
            .alias_authorization_rule_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let partner_namespace_id_binding = args.partner_namespace_id.get_inner();
        let primary_namespace_id_binding = args.primary_namespace_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:servicebus/namespaceDisasterRecoveryConfig:NamespaceDisasterRecoveryConfig"
                .into(),
            name: name.to_string(),
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
                    name: "partnerNamespaceId".into(),
                    value: &partner_namespace_id_binding,
                },
                register_interface::ObjectField {
                    name: "primaryNamespaceId".into(),
                    value: &primary_namespace_id_binding,
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
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "partnerNamespaceId".into(),
                },
                register_interface::ResultField {
                    name: "primaryConnectionStringAlias".into(),
                },
                register_interface::ResultField {
                    name: "primaryNamespaceId".into(),
                },
                register_interface::ResultField {
                    name: "secondaryConnectionStringAlias".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NamespaceDisasterRecoveryConfigResult {
            alias_authorization_rule_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aliasAuthorizationRuleId").unwrap(),
            ),
            default_primary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultPrimaryKey").unwrap(),
            ),
            default_secondary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSecondaryKey").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            partner_namespace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partnerNamespaceId").unwrap(),
            ),
            primary_connection_string_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryConnectionStringAlias").unwrap(),
            ),
            primary_namespace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryNamespaceId").unwrap(),
            ),
            secondary_connection_string_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryConnectionStringAlias").unwrap(),
            ),
        }
    }
}
