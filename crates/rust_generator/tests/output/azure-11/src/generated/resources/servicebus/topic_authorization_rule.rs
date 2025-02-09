/// Manages a ServiceBus Topic authorization Rule within a ServiceBus Topic.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: tfex-servicebus
///       location: West Europe
///   exampleNamespace:
///     type: azure:servicebus:Namespace
///     name: example
///     properties:
///       name: tfex-servicebus-namespace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Standard
///       tags:
///         source: example
///   exampleTopic:
///     type: azure:servicebus:Topic
///     name: example
///     properties:
///       name: tfex_servicebus_topic
///       namespaceId: ${exampleNamespace.id}
///   exampleTopicAuthorizationRule:
///     type: azure:servicebus:TopicAuthorizationRule
///     name: example
///     properties:
///       name: tfex_servicebus_topic_sasPolicy
///       topicId: ${exampleTopic.id}
///       listen: true
///       send: false
///       manage: false
/// ```
///
/// ## Import
///
/// ServiceBus Topic authorization rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:servicebus/topicAuthorizationRule:TopicAuthorizationRule rule1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ServiceBus/namespaces/namespace1/topics/topic1/authorizationRules/rule1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod topic_authorization_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TopicAuthorizationRuleArgs {
        /// Grants listen access to this this Authorization Rule. Defaults to `false`.
        #[builder(into, default)]
        pub listen: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Grants manage access to this this Authorization Rule. When this property is `true` - both `listen` and `send` must be too. Defaults to `false`.
        #[builder(into, default)]
        pub manage: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the name of the ServiceBus Topic Authorization Rule resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Grants send access to this this Authorization Rule. Defaults to `false`.
        #[builder(into, default)]
        pub send: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the ID of the ServiceBus Topic. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** At least one of the 3 permissions below needs to be set.
        #[builder(into)]
        pub topic_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TopicAuthorizationRuleResult {
        /// Grants listen access to this this Authorization Rule. Defaults to `false`.
        pub listen: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Grants manage access to this this Authorization Rule. When this property is `true` - both `listen` and `send` must be too. Defaults to `false`.
        pub manage: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name of the ServiceBus Topic Authorization Rule resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Primary Connection String for the ServiceBus Topic authorization Rule.
        pub primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The alias Primary Connection String for the ServiceBus Namespace, if the namespace is Geo DR paired.
        pub primary_connection_string_alias: pulumi_gestalt_rust::Output<String>,
        /// The Primary Key for the ServiceBus Topic authorization Rule.
        pub primary_key: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Connection String for the ServiceBus Topic authorization Rule.
        pub secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The alias Secondary Connection String for the ServiceBus Namespace
        pub secondary_connection_string_alias: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Key for the ServiceBus Topic authorization Rule.
        pub secondary_key: pulumi_gestalt_rust::Output<String>,
        /// Grants send access to this this Authorization Rule. Defaults to `false`.
        pub send: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the ID of the ServiceBus Topic. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** At least one of the 3 permissions below needs to be set.
        pub topic_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TopicAuthorizationRuleArgs,
    ) -> TopicAuthorizationRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let listen_binding_1 = args.listen.get_output(context);
        let listen_binding = listen_binding_1.get_inner();
        let manage_binding_1 = args.manage.get_output(context);
        let manage_binding = manage_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let send_binding_1 = args.send.get_output(context);
        let send_binding = send_binding_1.get_inner();
        let topic_id_binding_1 = args.topic_id.get_output(context);
        let topic_id_binding = topic_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:servicebus/topicAuthorizationRule:TopicAuthorizationRule"
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
                    name: "send".into(),
                    value: &send_binding,
                },
                register_interface::ObjectField {
                    name: "topicId".into(),
                    value: &topic_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TopicAuthorizationRuleResult {
            listen: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("listen"),
            ),
            manage: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("manage"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            primary_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryConnectionString"),
            ),
            primary_connection_string_alias: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryConnectionStringAlias"),
            ),
            primary_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryKey"),
            ),
            secondary_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryConnectionString"),
            ),
            secondary_connection_string_alias: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryConnectionStringAlias"),
            ),
            secondary_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryKey"),
            ),
            send: pulumi_gestalt_rust::__private::into_domain(o.extract_field("send")),
            topic_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("topicId"),
            ),
        }
    }
}
