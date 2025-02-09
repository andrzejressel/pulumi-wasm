#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_authorization_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAuthorizationRuleArgs {
        /// Specifies the name of the EventHub.
        #[builder(into)]
        pub eventhub_name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub listen: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub manage: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the name of the EventHub Authorization Rule resource. be created.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the grandparent EventHub Namespace.
        #[builder(into)]
        pub namespace_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which the EventHub Authorization Rule's grandparent Namespace exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub send: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetAuthorizationRuleResult {
        pub eventhub_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub listen: pulumi_gestalt_rust::Output<Option<bool>>,
        pub manage: pulumi_gestalt_rust::Output<Option<bool>>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub namespace_name: pulumi_gestalt_rust::Output<String>,
        /// The Primary Connection String for the Event Hubs Authorization Rule.
        pub primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The alias of the Primary Connection String for the Event Hubs Authorization Rule.
        pub primary_connection_string_alias: pulumi_gestalt_rust::Output<String>,
        /// The Primary Key for the Event Hubs Authorization Rule.
        pub primary_key: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Connection String for the Event Hubs Authorization Rule.
        pub secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The alias of the Secondary Connection String for the Event Hubs Authorization Rule.
        pub secondary_connection_string_alias: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Key for the Event Hubs Authorization Rule.
        pub secondary_key: pulumi_gestalt_rust::Output<String>,
        pub send: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAuthorizationRuleArgs,
    ) -> GetAuthorizationRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let eventhub_name_binding_1 = args.eventhub_name.get_output(context);
        let eventhub_name_binding = eventhub_name_binding_1.get_inner();
        let listen_binding_1 = args.listen.get_output(context);
        let listen_binding = listen_binding_1.get_inner();
        let manage_binding_1 = args.manage.get_output(context);
        let manage_binding = manage_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let namespace_name_binding_1 = args.namespace_name.get_output(context);
        let namespace_name_binding = namespace_name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let send_binding_1 = args.send.get_output(context);
        let send_binding = send_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:eventhub/getAuthorizationRule:getAuthorizationRule".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "eventhubName".into(),
                    value: &eventhub_name_binding,
                },
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
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAuthorizationRuleResult {
            eventhub_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventhubName"),
            ),
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
