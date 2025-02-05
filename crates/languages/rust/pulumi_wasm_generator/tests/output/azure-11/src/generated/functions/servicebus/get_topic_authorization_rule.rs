pub mod get_topic_authorization_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTopicAuthorizationRuleArgs {
        /// The name of the ServiceBus Topic Authorization Rule resource.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the ServiceBus Namespace.
        #[builder(into, default)]
        pub namespace_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub queue_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the ServiceBus Namespace exists.
        #[builder(into, default)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub topic_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the ServiceBus Topic.
        #[builder(into, default)]
        pub topic_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetTopicAuthorizationRuleResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub listen: pulumi_wasm_rust::Output<bool>,
        pub manage: pulumi_wasm_rust::Output<bool>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub namespace_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Primary Connection String for the ServiceBus Topic authorization Rule.
        pub primary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The alias Primary Connection String for the ServiceBus Namespace, if the namespace is Geo DR paired.
        pub primary_connection_string_alias: pulumi_wasm_rust::Output<String>,
        /// The Primary Key for the ServiceBus Topic authorization Rule.
        pub primary_key: pulumi_wasm_rust::Output<String>,
        pub queue_name: pulumi_wasm_rust::Output<Option<String>>,
        pub resource_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Secondary Connection String for the ServiceBus Topic authorization Rule.
        pub secondary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The alias Secondary Connection String for the ServiceBus Namespace
        pub secondary_connection_string_alias: pulumi_wasm_rust::Output<String>,
        /// The Secondary Key for the ServiceBus Topic authorization Rule.
        pub secondary_key: pulumi_wasm_rust::Output<String>,
        pub send: pulumi_wasm_rust::Output<bool>,
        pub topic_id: pulumi_wasm_rust::Output<Option<String>>,
        pub topic_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetTopicAuthorizationRuleArgs,
    ) -> GetTopicAuthorizationRuleResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let namespace_name_binding = args.namespace_name.get_output(context).get_inner();
        let queue_name_binding = args.queue_name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let topic_id_binding = args.topic_id.get_output(context).get_inner();
        let topic_name_binding = args.topic_name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:servicebus/getTopicAuthorizationRule:getTopicAuthorizationRule"
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
                    name: "queueName".into(),
                    value: &queue_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "topicId".into(),
                    value: &topic_id_binding,
                },
                register_interface::ObjectField {
                    name: "topicName".into(),
                    value: &topic_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetTopicAuthorizationRuleResult {
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
            queue_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("queueName"),
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
            topic_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("topicId"),
            ),
            topic_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("topicName"),
            ),
        }
    }
}
