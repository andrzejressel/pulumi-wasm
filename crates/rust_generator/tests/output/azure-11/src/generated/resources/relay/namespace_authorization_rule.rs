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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod namespace_authorization_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NamespaceAuthorizationRuleArgs {
        /// Grants listen access to this Authorization Rule. Defaults to `false`.
        #[builder(into, default)]
        pub listen: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Grants manage access to this Authorization Rule. When this property is `true` - both `listen` and `send` must be set to `true` too. Defaults to `false`.
        #[builder(into, default)]
        pub manage: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name which should be used for this Azure Relay Namespace Authorization Rule. Changing this forces a new Azure Relay Namespace Authorization Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the Azure Relay Namespace for which this Azure Relay Namespace Authorization Rule will be created. Changing this forces a new Azure Relay Namespace Authorization Rule to be created.
        #[builder(into)]
        pub namespace_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Azure Relay Namespace Authorization Rule should exist. Changing this forces a new Azure Relay Namespace Authorization Rule to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Grants send access to this Authorization Rule. Defaults to `false`.
        #[builder(into, default)]
        pub send: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct NamespaceAuthorizationRuleResult {
        /// Grants listen access to this Authorization Rule. Defaults to `false`.
        pub listen: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Grants manage access to this Authorization Rule. When this property is `true` - both `listen` and `send` must be set to `true` too. Defaults to `false`.
        pub manage: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name which should be used for this Azure Relay Namespace Authorization Rule. Changing this forces a new Azure Relay Namespace Authorization Rule to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Name of the Azure Relay Namespace for which this Azure Relay Namespace Authorization Rule will be created. Changing this forces a new Azure Relay Namespace Authorization Rule to be created.
        pub namespace_name: pulumi_gestalt_rust::Output<String>,
        /// The Primary Connection String for the Azure Relay Namespace Authorization Rule.
        pub primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The Primary Key for the Azure Relay Namespace Authorization Rule.
        pub primary_key: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Azure Relay Namespace Authorization Rule should exist. Changing this forces a new Azure Relay Namespace Authorization Rule to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Connection String for the Azure Relay Namespace Authorization Rule.
        pub secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Key for the Azure Relay Namespace Authorization Rule.
        pub secondary_key: pulumi_gestalt_rust::Output<String>,
        /// Grants send access to this Authorization Rule. Defaults to `false`.
        pub send: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: NamespaceAuthorizationRuleArgs,
    ) -> NamespaceAuthorizationRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let listen_binding = args.listen.get_output(context).get_inner();
        let manage_binding = args.manage.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let namespace_name_binding = args.namespace_name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let send_binding = args.send.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:relay/namespaceAuthorizationRule:NamespaceAuthorizationRule"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        NamespaceAuthorizationRuleResult {
            listen: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("listen"),
            ),
            manage: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("manage"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            namespace_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namespaceName"),
            ),
            primary_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryConnectionString"),
            ),
            primary_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryKey"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            secondary_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryConnectionString"),
            ),
            secondary_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryKey"),
            ),
            send: pulumi_gestalt_rust::__private::into_domain(o.extract_field("send")),
        }
    }
}
