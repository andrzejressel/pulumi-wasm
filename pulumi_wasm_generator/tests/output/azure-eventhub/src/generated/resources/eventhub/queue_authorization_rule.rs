/// Manages an Authorization Rule for a ServiceBus Queue.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: my-servicebus
///       location: West US
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
///   exampleQueue:
///     type: azure:servicebus:Queue
///     name: example
///     properties:
///       name: tfex_servicebus_queue
///       namespaceId: ${exampleNamespace.id}
///       enablePartitioning: true
///   exampleQueueAuthorizationRule:
///     type: azure:servicebus:QueueAuthorizationRule
///     name: example
///     properties:
///       name: examplerule
///       queueId: ${exampleQueue.id}
///       listen: true
///       send: true
///       manage: false
/// ```
///
/// ## Import
///
/// ServiceBus Queue Authorization Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventhub/queueAuthorizationRule:QueueAuthorizationRule rule1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ServiceBus/namespaces/namespace1/queues/queue1/authorizationRules/rule1
/// ```
///
pub mod queue_authorization_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct QueueAuthorizationRuleArgs {
        /// Does this Authorization Rule have Listen permissions to the ServiceBus Queue? Defaults to `false`.
        #[builder(into, default)]
        pub listen: pulumi_wasm_rust::Output<Option<bool>>,
        /// Does this Authorization Rule have Manage permissions to the ServiceBus Queue? When this property is `true` - both `listen` and `send` must be too. Defaults to `false`.
        #[builder(into, default)]
        pub manage: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the name of the Authorization Rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the ServiceBus Queue. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** At least one of the 3 permissions below needs to be set.
        #[builder(into)]
        pub queue_id: pulumi_wasm_rust::Output<String>,
        /// Does this Authorization Rule have Send permissions to the ServiceBus Queue? Defaults to `false`.
        #[builder(into, default)]
        pub send: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct QueueAuthorizationRuleResult {
        /// Does this Authorization Rule have Listen permissions to the ServiceBus Queue? Defaults to `false`.
        pub listen: pulumi_wasm_rust::Output<Option<bool>>,
        /// Does this Authorization Rule have Manage permissions to the ServiceBus Queue? When this property is `true` - both `listen` and `send` must be too. Defaults to `false`.
        pub manage: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the name of the Authorization Rule. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Primary Connection String for the Authorization Rule.
        pub primary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The alias Primary Connection String for the ServiceBus Namespace, if the namespace is Geo DR paired.
        pub primary_connection_string_alias: pulumi_wasm_rust::Output<String>,
        /// The Primary Key for the Authorization Rule.
        pub primary_key: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the ServiceBus Queue. Changing this forces a new resource to be created.
        ///
        /// > **NOTE** At least one of the 3 permissions below needs to be set.
        pub queue_id: pulumi_wasm_rust::Output<String>,
        /// The Secondary Connection String for the Authorization Rule.
        pub secondary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The alias Secondary Connection String for the ServiceBus Namespace
        pub secondary_connection_string_alias: pulumi_wasm_rust::Output<String>,
        /// The Secondary Key for the Authorization Rule.
        pub secondary_key: pulumi_wasm_rust::Output<String>,
        /// Does this Authorization Rule have Send permissions to the ServiceBus Queue? Defaults to `false`.
        pub send: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: QueueAuthorizationRuleArgs,
    ) -> QueueAuthorizationRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let listen_binding = args.listen.get_inner();
        let manage_binding = args.manage.get_inner();
        let name_binding = args.name.get_inner();
        let queue_id_binding = args.queue_id.get_inner();
        let send_binding = args.send.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:eventhub/queueAuthorizationRule:QueueAuthorizationRule".into(),
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
                    name: "queueId".into(),
                    value: &queue_id_binding,
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
                    name: "primaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "primaryConnectionStringAlias".into(),
                },
                register_interface::ResultField {
                    name: "primaryKey".into(),
                },
                register_interface::ResultField {
                    name: "queueId".into(),
                },
                register_interface::ResultField {
                    name: "secondaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "secondaryConnectionStringAlias".into(),
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
        QueueAuthorizationRuleResult {
            listen: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("listen").unwrap(),
            ),
            manage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("manage").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            primary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryConnectionString").unwrap(),
            ),
            primary_connection_string_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryConnectionStringAlias").unwrap(),
            ),
            primary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryKey").unwrap(),
            ),
            queue_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queueId").unwrap(),
            ),
            secondary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryConnectionString").unwrap(),
            ),
            secondary_connection_string_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryConnectionStringAlias").unwrap(),
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
