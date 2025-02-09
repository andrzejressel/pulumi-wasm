#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_queue_authorization_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetQueueAuthorizationRuleArgs {
        /// The name of this ServiceBus Queue Authorisation Rule.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the ServiceBus Namespace.
        #[builder(into, default)]
        pub namespace_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub queue_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the ServiceBus Queue.
        #[builder(into, default)]
        pub queue_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the ServiceBus Queue Authorisation Rule exists.
        #[builder(into, default)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetQueueAuthorizationRuleResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub listen: pulumi_gestalt_rust::Output<bool>,
        pub manage: pulumi_gestalt_rust::Output<bool>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub namespace_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Primary Connection String for the ServiceBus Queue authorization Rule.
        pub primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The alias Primary Connection String for the ServiceBus Namespace, if the namespace is Geo DR paired.
        pub primary_connection_string_alias: pulumi_gestalt_rust::Output<String>,
        /// The Primary Key for the ServiceBus Queue authorization Rule.
        pub primary_key: pulumi_gestalt_rust::Output<String>,
        pub queue_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub queue_name: pulumi_gestalt_rust::Output<Option<String>>,
        pub resource_group_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Secondary Connection String for the ServiceBus Queue authorization Rule.
        pub secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The alias Secondary Connection String for the ServiceBus Namespace
        pub secondary_connection_string_alias: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Key for the ServiceBus Queue authorization Rule.
        pub secondary_key: pulumi_gestalt_rust::Output<String>,
        pub send: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetQueueAuthorizationRuleArgs,
    ) -> GetQueueAuthorizationRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let namespace_name_binding_1 = args.namespace_name.get_output(context);
        let namespace_name_binding = namespace_name_binding_1.get_inner();
        let queue_id_binding_1 = args.queue_id.get_output(context);
        let queue_id_binding = queue_id_binding_1.get_inner();
        let queue_name_binding_1 = args.queue_name.get_output(context);
        let queue_name_binding = queue_name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:servicebus/getQueueAuthorizationRule:getQueueAuthorizationRule"
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
                    name: "queueId".into(),
                    value: &queue_id_binding,
                },
                register_interface::ObjectField {
                    name: "queueName".into(),
                    value: &queue_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetQueueAuthorizationRuleResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
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
            primary_connection_string_alias: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryConnectionStringAlias"),
            ),
            primary_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryKey"),
            ),
            queue_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("queueId"),
            ),
            queue_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("queueName"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
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
        }
    }
}
