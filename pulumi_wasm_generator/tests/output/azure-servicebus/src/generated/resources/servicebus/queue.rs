/// Manages a ServiceBus Queue.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: my-servicebus
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
///   exampleQueue:
///     type: azure:servicebus:Queue
///     name: example
///     properties:
///       name: tfex_servicebus_queue
///       namespaceId: ${exampleNamespace.id}
///       partitioningEnabled: true
/// ```
///
/// ## Import
///
/// Service Bus Queue can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:servicebus/queue:Queue example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ServiceBus/namespaces/sbns1/queues/snqueue1
/// ```
///
pub mod queue {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct QueueArgs {
        /// The ISO 8601 timespan duration of the idle interval after which the Queue is automatically deleted, minimum of 5 minutes.
        #[builder(into, default)]
        pub auto_delete_on_idle: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean flag which controls whether server-side batched operations are enabled. Defaults to `true`.
        #[builder(into, default)]
        pub batched_operations_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean flag which controls whether the Queue has dead letter support when a message expires. Defaults to `false`.
        #[builder(into, default)]
        pub dead_lettering_on_message_expiration: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ISO 8601 timespan duration of the TTL of messages sent to this queue. This is the default value used when TTL is not set on message itself.
        #[builder(into, default)]
        pub default_message_ttl: pulumi_wasm_rust::Output<Option<String>>,
        /// The ISO 8601 timespan duration during which duplicates can be detected. Defaults to `PT10M` (10 Minutes).
        #[builder(into, default)]
        pub duplicate_detection_history_time_window: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// Boolean flag which controls whether Express Entities are enabled. An express queue holds a message in memory temporarily before writing it to persistent storage. Defaults to `false` for Basic and Standard. For Premium, it MUST be set to `false`.
        ///
        /// > **NOTE:** Service Bus Premium namespaces do not support Express Entities, so `express_enabled` MUST be set to `false`.
        #[builder(into, default)]
        pub express_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of a Queue or Topic to automatically forward dead lettered messages to.
        #[builder(into, default)]
        pub forward_dead_lettered_messages_to: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of a Queue or Topic to automatically forward messages to. Please [see the documentation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-auto-forwarding) for more information.
        #[builder(into, default)]
        pub forward_to: pulumi_wasm_rust::Output<Option<String>>,
        /// The ISO 8601 timespan duration of a peek-lock; that is, the amount of time that the message is locked for other receivers. Maximum value is 5 minutes. Defaults to `PT1M` (1 Minute).
        #[builder(into, default)]
        pub lock_duration: pulumi_wasm_rust::Output<Option<String>>,
        /// Integer value which controls when a message is automatically dead lettered. Defaults to `10`.
        #[builder(into, default)]
        pub max_delivery_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Integer value which controls the maximum size of a message allowed on the queue for Premium SKU. For supported values see the "Large messages support" section of [this document](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-premium-messaging#large-messages-support-preview).
        #[builder(into, default)]
        pub max_message_size_in_kilobytes: pulumi_wasm_rust::Output<Option<i32>>,
        /// Integer value which controls the size of memory allocated for the queue. For supported values see the "Queue or topic size" section of [Service Bus Quotas](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-quotas).
        #[builder(into, default)]
        pub max_size_in_megabytes: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the name of the ServiceBus Queue resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the ServiceBus Namespace to create this queue in. Changing this forces a new resource to be created.
        #[builder(into)]
        pub namespace_id: pulumi_wasm_rust::Output<String>,
        /// Boolean flag which controls whether to enable the queue to be partitioned across multiple message brokers. Changing this forces a new resource to be created. Defaults to `false` for Basic and Standard.
        ///
        /// > **NOTE:** Partitioning is available at entity creation for all queues and topics in Basic or Standard SKUs. For premium namespace, partitioning is available at namespace creation, and all queues and topics in the partitioned namespace will be partitioned, for the premium namespace that has `premium_messaging_partitions` sets to `1`, the namespace is not partitioned.
        #[builder(into, default)]
        pub partitioning_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean flag which controls whether the Queue requires duplicate detection. Changing this forces a new resource to be created. Defaults to `false`.
        #[builder(into, default)]
        pub requires_duplicate_detection: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean flag which controls whether the Queue requires sessions. This will allow ordered handling of unbounded sequences of related messages. With sessions enabled a queue can guarantee first-in-first-out delivery of messages. Changing this forces a new resource to be created. Defaults to `false`.
        #[builder(into, default)]
        pub requires_session: pulumi_wasm_rust::Output<Option<bool>>,
        /// The status of the Queue. Possible values are `Active`, `Creating`, `Deleting`, `Disabled`, `ReceiveDisabled`, `Renaming`, `SendDisabled`, `Unknown`. Note that `Restoring` is not accepted. Defaults to `Active`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct QueueResult {
        /// The ISO 8601 timespan duration of the idle interval after which the Queue is automatically deleted, minimum of 5 minutes.
        pub auto_delete_on_idle: pulumi_wasm_rust::Output<String>,
        /// Boolean flag which controls whether server-side batched operations are enabled. Defaults to `true`.
        pub batched_operations_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean flag which controls whether the Queue has dead letter support when a message expires. Defaults to `false`.
        pub dead_lettering_on_message_expiration: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ISO 8601 timespan duration of the TTL of messages sent to this queue. This is the default value used when TTL is not set on message itself.
        pub default_message_ttl: pulumi_wasm_rust::Output<String>,
        /// The ISO 8601 timespan duration during which duplicates can be detected. Defaults to `PT10M` (10 Minutes).
        pub duplicate_detection_history_time_window: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// Boolean flag which controls whether Express Entities are enabled. An express queue holds a message in memory temporarily before writing it to persistent storage. Defaults to `false` for Basic and Standard. For Premium, it MUST be set to `false`.
        ///
        /// > **NOTE:** Service Bus Premium namespaces do not support Express Entities, so `express_enabled` MUST be set to `false`.
        pub express_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of a Queue or Topic to automatically forward dead lettered messages to.
        pub forward_dead_lettered_messages_to: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of a Queue or Topic to automatically forward messages to. Please [see the documentation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-auto-forwarding) for more information.
        pub forward_to: pulumi_wasm_rust::Output<Option<String>>,
        /// The ISO 8601 timespan duration of a peek-lock; that is, the amount of time that the message is locked for other receivers. Maximum value is 5 minutes. Defaults to `PT1M` (1 Minute).
        pub lock_duration: pulumi_wasm_rust::Output<Option<String>>,
        /// Integer value which controls when a message is automatically dead lettered. Defaults to `10`.
        pub max_delivery_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Integer value which controls the maximum size of a message allowed on the queue for Premium SKU. For supported values see the "Large messages support" section of [this document](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-premium-messaging#large-messages-support-preview).
        pub max_message_size_in_kilobytes: pulumi_wasm_rust::Output<i32>,
        /// Integer value which controls the size of memory allocated for the queue. For supported values see the "Queue or topic size" section of [Service Bus Quotas](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-quotas).
        pub max_size_in_megabytes: pulumi_wasm_rust::Output<i32>,
        /// Specifies the name of the ServiceBus Queue resource. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the ServiceBus Namespace to create this queue in. Changing this forces a new resource to be created.
        pub namespace_id: pulumi_wasm_rust::Output<String>,
        pub namespace_name: pulumi_wasm_rust::Output<String>,
        /// Boolean flag which controls whether to enable the queue to be partitioned across multiple message brokers. Changing this forces a new resource to be created. Defaults to `false` for Basic and Standard.
        ///
        /// > **NOTE:** Partitioning is available at entity creation for all queues and topics in Basic or Standard SKUs. For premium namespace, partitioning is available at namespace creation, and all queues and topics in the partitioned namespace will be partitioned, for the premium namespace that has `premium_messaging_partitions` sets to `1`, the namespace is not partitioned.
        pub partitioning_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean flag which controls whether the Queue requires duplicate detection. Changing this forces a new resource to be created. Defaults to `false`.
        pub requires_duplicate_detection: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean flag which controls whether the Queue requires sessions. This will allow ordered handling of unbounded sequences of related messages. With sessions enabled a queue can guarantee first-in-first-out delivery of messages. Changing this forces a new resource to be created. Defaults to `false`.
        pub requires_session: pulumi_wasm_rust::Output<Option<bool>>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The status of the Queue. Possible values are `Active`, `Creating`, `Deleting`, `Disabled`, `ReceiveDisabled`, `Renaming`, `SendDisabled`, `Unknown`. Note that `Restoring` is not accepted. Defaults to `Active`.
        pub status: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: QueueArgs) -> QueueResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_delete_on_idle_binding = args.auto_delete_on_idle.get_inner();
        let batched_operations_enabled_binding = args
            .batched_operations_enabled
            .get_inner();
        let dead_lettering_on_message_expiration_binding = args
            .dead_lettering_on_message_expiration
            .get_inner();
        let default_message_ttl_binding = args.default_message_ttl.get_inner();
        let duplicate_detection_history_time_window_binding = args
            .duplicate_detection_history_time_window
            .get_inner();
        let express_enabled_binding = args.express_enabled.get_inner();
        let forward_dead_lettered_messages_to_binding = args
            .forward_dead_lettered_messages_to
            .get_inner();
        let forward_to_binding = args.forward_to.get_inner();
        let lock_duration_binding = args.lock_duration.get_inner();
        let max_delivery_count_binding = args.max_delivery_count.get_inner();
        let max_message_size_in_kilobytes_binding = args
            .max_message_size_in_kilobytes
            .get_inner();
        let max_size_in_megabytes_binding = args.max_size_in_megabytes.get_inner();
        let name_binding = args.name.get_inner();
        let namespace_id_binding = args.namespace_id.get_inner();
        let partitioning_enabled_binding = args.partitioning_enabled.get_inner();
        let requires_duplicate_detection_binding = args
            .requires_duplicate_detection
            .get_inner();
        let requires_session_binding = args.requires_session.get_inner();
        let status_binding = args.status.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:servicebus/queue:Queue".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoDeleteOnIdle".into(),
                    value: &auto_delete_on_idle_binding,
                },
                register_interface::ObjectField {
                    name: "batchedOperationsEnabled".into(),
                    value: &batched_operations_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "deadLetteringOnMessageExpiration".into(),
                    value: &dead_lettering_on_message_expiration_binding,
                },
                register_interface::ObjectField {
                    name: "defaultMessageTtl".into(),
                    value: &default_message_ttl_binding,
                },
                register_interface::ObjectField {
                    name: "duplicateDetectionHistoryTimeWindow".into(),
                    value: &duplicate_detection_history_time_window_binding,
                },
                register_interface::ObjectField {
                    name: "expressEnabled".into(),
                    value: &express_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "forwardDeadLetteredMessagesTo".into(),
                    value: &forward_dead_lettered_messages_to_binding,
                },
                register_interface::ObjectField {
                    name: "forwardTo".into(),
                    value: &forward_to_binding,
                },
                register_interface::ObjectField {
                    name: "lockDuration".into(),
                    value: &lock_duration_binding,
                },
                register_interface::ObjectField {
                    name: "maxDeliveryCount".into(),
                    value: &max_delivery_count_binding,
                },
                register_interface::ObjectField {
                    name: "maxMessageSizeInKilobytes".into(),
                    value: &max_message_size_in_kilobytes_binding,
                },
                register_interface::ObjectField {
                    name: "maxSizeInMegabytes".into(),
                    value: &max_size_in_megabytes_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namespaceId".into(),
                    value: &namespace_id_binding,
                },
                register_interface::ObjectField {
                    name: "partitioningEnabled".into(),
                    value: &partitioning_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "requiresDuplicateDetection".into(),
                    value: &requires_duplicate_detection_binding,
                },
                register_interface::ObjectField {
                    name: "requiresSession".into(),
                    value: &requires_session_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoDeleteOnIdle".into(),
                },
                register_interface::ResultField {
                    name: "batchedOperationsEnabled".into(),
                },
                register_interface::ResultField {
                    name: "deadLetteringOnMessageExpiration".into(),
                },
                register_interface::ResultField {
                    name: "defaultMessageTtl".into(),
                },
                register_interface::ResultField {
                    name: "duplicateDetectionHistoryTimeWindow".into(),
                },
                register_interface::ResultField {
                    name: "expressEnabled".into(),
                },
                register_interface::ResultField {
                    name: "forwardDeadLetteredMessagesTo".into(),
                },
                register_interface::ResultField {
                    name: "forwardTo".into(),
                },
                register_interface::ResultField {
                    name: "lockDuration".into(),
                },
                register_interface::ResultField {
                    name: "maxDeliveryCount".into(),
                },
                register_interface::ResultField {
                    name: "maxMessageSizeInKilobytes".into(),
                },
                register_interface::ResultField {
                    name: "maxSizeInMegabytes".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namespaceId".into(),
                },
                register_interface::ResultField {
                    name: "namespaceName".into(),
                },
                register_interface::ResultField {
                    name: "partitioningEnabled".into(),
                },
                register_interface::ResultField {
                    name: "requiresDuplicateDetection".into(),
                },
                register_interface::ResultField {
                    name: "requiresSession".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        QueueResult {
            auto_delete_on_idle: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoDeleteOnIdle").unwrap(),
            ),
            batched_operations_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("batchedOperationsEnabled").unwrap(),
            ),
            dead_lettering_on_message_expiration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deadLetteringOnMessageExpiration").unwrap(),
            ),
            default_message_ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultMessageTtl").unwrap(),
            ),
            duplicate_detection_history_time_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("duplicateDetectionHistoryTimeWindow").unwrap(),
            ),
            express_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expressEnabled").unwrap(),
            ),
            forward_dead_lettered_messages_to: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forwardDeadLetteredMessagesTo").unwrap(),
            ),
            forward_to: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forwardTo").unwrap(),
            ),
            lock_duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lockDuration").unwrap(),
            ),
            max_delivery_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxDeliveryCount").unwrap(),
            ),
            max_message_size_in_kilobytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxMessageSizeInKilobytes").unwrap(),
            ),
            max_size_in_megabytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxSizeInMegabytes").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            namespace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespaceId").unwrap(),
            ),
            namespace_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespaceName").unwrap(),
            ),
            partitioning_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partitioningEnabled").unwrap(),
            ),
            requires_duplicate_detection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requiresDuplicateDetection").unwrap(),
            ),
            requires_session: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requiresSession").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}