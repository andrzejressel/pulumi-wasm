#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_namespace_disaster_recovery_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNamespaceDisasterRecoveryConfigArgs {
        #[builder(into, default)]
        pub alias_authorization_rule_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub namespace_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub namespace_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetNamespaceDisasterRecoveryConfigResult {
        pub alias_authorization_rule_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub default_primary_key: pulumi_gestalt_rust::Output<String>,
        pub default_secondary_key: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub namespace_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub namespace_name: pulumi_gestalt_rust::Output<Option<String>>,
        pub partner_namespace_id: pulumi_gestalt_rust::Output<String>,
        pub primary_connection_string_alias: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<Option<String>>,
        pub secondary_connection_string_alias: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNamespaceDisasterRecoveryConfigArgs,
    ) -> GetNamespaceDisasterRecoveryConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let alias_authorization_rule_id_binding = args
            .alias_authorization_rule_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let namespace_id_binding = args.namespace_id.get_output(context);
        let namespace_name_binding = args.namespace_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:servicebus/getNamespaceDisasterRecoveryConfig:getNamespaceDisasterRecoveryConfig"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "aliasAuthorizationRuleId".into(),
                    value: &alias_authorization_rule_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespaceId".into(),
                    value: &namespace_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespaceName".into(),
                    value: &namespace_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNamespaceDisasterRecoveryConfigResult {
            alias_authorization_rule_id: o.get_field("aliasAuthorizationRuleId"),
            default_primary_key: o.get_field("defaultPrimaryKey"),
            default_secondary_key: o.get_field("defaultSecondaryKey"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            namespace_id: o.get_field("namespaceId"),
            namespace_name: o.get_field("namespaceName"),
            partner_namespace_id: o.get_field("partnerNamespaceId"),
            primary_connection_string_alias: o.get_field("primaryConnectionStringAlias"),
            resource_group_name: o.get_field("resourceGroupName"),
            secondary_connection_string_alias: o
                .get_field("secondaryConnectionStringAlias"),
        }
    }
}
