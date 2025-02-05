pub mod get_queue {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetQueueArgs {
        /// The name of this Service Bus Queue.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the ServiceBus Namespace where the Service Bus Queue exists.
        #[builder(into, default)]
        pub namespace_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub namespace_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetQueueResult {
        /// The ISO 8601 timespan duration of the idle interval after which the Queue is automatically deleted, minimum of 5 minutes.
        pub auto_delete_on_idle: pulumi_wasm_rust::Output<String>,
        /// Boolean flag which controls whether the Queue has dead letter support when a message expires.
        pub dead_lettering_on_message_expiration: pulumi_wasm_rust::Output<bool>,
        /// The ISO 8601 timespan duration of the TTL of messages sent to this queue. This is the default value used when TTL is not set on a message itself.
        pub default_message_ttl: pulumi_wasm_rust::Output<String>,
        /// The ISO 8601 timespan duration during which duplicates can be detected.
        pub duplicate_detection_history_time_window: pulumi_wasm_rust::Output<String>,
        /// Boolean flag which controls whether server-side batched operations are enabled.
        pub enable_batched_operations: pulumi_wasm_rust::Output<bool>,
        /// Boolean flag which controls whether Express Entities are enabled. An express queue holds a message in memory temporarily before writing it to persistent storage.
        pub enable_express: pulumi_wasm_rust::Output<bool>,
        /// Boolean flag which controls whether to enable the queue to be partitioned across multiple message brokers.
        pub enable_partitioning: pulumi_wasm_rust::Output<bool>,
        /// The name of a Queue or Topic to automatically forward dead lettered messages to.
        pub forward_dead_lettered_messages_to: pulumi_wasm_rust::Output<String>,
        /// The name of a Queue or Topic to automatically forward messages to. Please [see the documentation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-auto-forwarding) for more information.
        pub forward_to: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The ISO 8601 timespan duration of a peek-lock; that is, the amount of time that the message is locked for other receivers.
        pub lock_duration: pulumi_wasm_rust::Output<String>,
        /// Integer value which controls when a message is automatically dead lettered.
        pub max_delivery_count: pulumi_wasm_rust::Output<i32>,
        /// Integer value which controls the size of memory allocated for the queue. For supported values see the "Queue or topic size" section of [Service Bus Quotas](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-quotas).
        pub max_size_in_megabytes: pulumi_wasm_rust::Output<i32>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub namespace_id: pulumi_wasm_rust::Output<Option<String>>,
        pub namespace_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean flag which controls whether the Queue requires duplicate detection.
        pub requires_duplicate_detection: pulumi_wasm_rust::Output<bool>,
        /// Boolean flag which controls whether the Queue requires sessions. This will allow ordered handling of unbounded sequences of related messages. With sessions enabled a queue can guarantee first-in-first-out delivery of messages.
        pub requires_session: pulumi_wasm_rust::Output<bool>,
        pub resource_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The status of the Queue. Possible values are `Active`, `Creating`, `Deleting`, `Disabled`, `ReceiveDisabled`, `Renaming`, `SendDisabled`, `Unknown`.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetQueueArgs,
    ) -> GetQueueResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let namespace_id_binding = args.namespace_id.get_output(context).get_inner();
        let namespace_name_binding = args.namespace_name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:servicebus/getQueue:getQueue".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namespaceId".into(),
                    value: &namespace_id_binding,
                },
                register_interface::ObjectField {
                    name: "namespaceName".into(),
                    value: &namespace_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetQueueResult {
            auto_delete_on_idle: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoDeleteOnIdle"),
            ),
            dead_lettering_on_message_expiration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deadLetteringOnMessageExpiration"),
            ),
            default_message_ttl: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultMessageTtl"),
            ),
            duplicate_detection_history_time_window: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("duplicateDetectionHistoryTimeWindow"),
            ),
            enable_batched_operations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enableBatchedOperations"),
            ),
            enable_express: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enableExpress"),
            ),
            enable_partitioning: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enablePartitioning"),
            ),
            forward_dead_lettered_messages_to: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("forwardDeadLetteredMessagesTo"),
            ),
            forward_to: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("forwardTo"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            lock_duration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lockDuration"),
            ),
            max_delivery_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maxDeliveryCount"),
            ),
            max_size_in_megabytes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maxSizeInMegabytes"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            namespace_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("namespaceId"),
            ),
            namespace_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("namespaceName"),
            ),
            requires_duplicate_detection: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("requiresDuplicateDetection"),
            ),
            requires_session: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("requiresSession"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
        }
    }
}
