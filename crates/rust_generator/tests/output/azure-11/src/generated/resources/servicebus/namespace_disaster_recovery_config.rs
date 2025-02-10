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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod namespace_disaster_recovery_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NamespaceDisasterRecoveryConfigArgs {
        /// The Shared access policies used to access the connection string for the alias.
        #[builder(into, default)]
        pub alias_authorization_rule_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the name of the Disaster Recovery Config. This is the alias DNS name that will be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Service Bus Namespace to replicate to.
        #[builder(into)]
        pub partner_namespace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the primary Service Bus Namespace to replicate. Changing this forces a new resource to be created.
        #[builder(into)]
        pub primary_namespace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NamespaceDisasterRecoveryConfigResult {
        /// The Shared access policies used to access the connection string for the alias.
        pub alias_authorization_rule_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The primary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_primary_key: pulumi_gestalt_rust::Output<String>,
        /// The secondary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_key: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Disaster Recovery Config. This is the alias DNS name that will be created. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Service Bus Namespace to replicate to.
        pub partner_namespace_id: pulumi_gestalt_rust::Output<String>,
        /// The alias Primary Connection String for the ServiceBus Namespace.
        pub primary_connection_string_alias: pulumi_gestalt_rust::Output<String>,
        /// The ID of the primary Service Bus Namespace to replicate. Changing this forces a new resource to be created.
        pub primary_namespace_id: pulumi_gestalt_rust::Output<String>,
        /// The alias Secondary Connection String for the ServiceBus Namespace
        pub secondary_connection_string_alias: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NamespaceDisasterRecoveryConfigArgs,
    ) -> NamespaceDisasterRecoveryConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let alias_authorization_rule_id_binding = args
            .alias_authorization_rule_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let partner_namespace_id_binding = args.partner_namespace_id.get_output(context);
        let primary_namespace_id_binding = args.primary_namespace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:servicebus/namespaceDisasterRecoveryConfig:NamespaceDisasterRecoveryConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "aliasAuthorizationRuleId".into(),
                    value: alias_authorization_rule_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "partnerNamespaceId".into(),
                    value: partner_namespace_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "primaryNamespaceId".into(),
                    value: primary_namespace_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NamespaceDisasterRecoveryConfigResult {
            alias_authorization_rule_id: o.get_field("aliasAuthorizationRuleId"),
            default_primary_key: o.get_field("defaultPrimaryKey"),
            default_secondary_key: o.get_field("defaultSecondaryKey"),
            name: o.get_field("name"),
            partner_namespace_id: o.get_field("partnerNamespaceId"),
            primary_connection_string_alias: o.get_field("primaryConnectionStringAlias"),
            primary_namespace_id: o.get_field("primaryNamespaceId"),
            secondary_connection_string_alias: o
                .get_field("secondaryConnectionStringAlias"),
        }
    }
}
