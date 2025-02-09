#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_queue {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetQueueArgs {
        /// The name of this Service Bus Queue.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the ServiceBus Namespace where the Service Bus Queue exists.
        #[builder(into, default)]
        pub namespace_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub namespace_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetQueueResult {
        /// The ISO 8601 timespan duration of the idle interval after which the Queue is automatically deleted, minimum of 5 minutes.
        pub auto_delete_on_idle: pulumi_gestalt_rust::Output<String>,
        /// Boolean flag which controls whether the Queue has dead letter support when a message expires.
        pub dead_lettering_on_message_expiration: pulumi_gestalt_rust::Output<bool>,
        /// The ISO 8601 timespan duration of the TTL of messages sent to this queue. This is the default value used when TTL is not set on a message itself.
        pub default_message_ttl: pulumi_gestalt_rust::Output<String>,
        /// The ISO 8601 timespan duration during which duplicates can be detected.
        pub duplicate_detection_history_time_window: pulumi_gestalt_rust::Output<String>,
        /// Boolean flag which controls whether server-side batched operations are enabled.
        pub enable_batched_operations: pulumi_gestalt_rust::Output<bool>,
        /// Boolean flag which controls whether Express Entities are enabled. An express queue holds a message in memory temporarily before writing it to persistent storage.
        pub enable_express: pulumi_gestalt_rust::Output<bool>,
        /// Boolean flag which controls whether to enable the queue to be partitioned across multiple message brokers.
        pub enable_partitioning: pulumi_gestalt_rust::Output<bool>,
        /// The name of a Queue or Topic to automatically forward dead lettered messages to.
        pub forward_dead_lettered_messages_to: pulumi_gestalt_rust::Output<String>,
        /// The name of a Queue or Topic to automatically forward messages to. Please [see the documentation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-auto-forwarding) for more information.
        pub forward_to: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ISO 8601 timespan duration of a peek-lock; that is, the amount of time that the message is locked for other receivers.
        pub lock_duration: pulumi_gestalt_rust::Output<String>,
        /// Integer value which controls when a message is automatically dead lettered.
        pub max_delivery_count: pulumi_gestalt_rust::Output<i32>,
        /// Integer value which controls the size of memory allocated for the queue. For supported values see the "Queue or topic size" section of [Service Bus Quotas](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-quotas).
        pub max_size_in_megabytes: pulumi_gestalt_rust::Output<i32>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub namespace_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub namespace_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Boolean flag which controls whether the Queue requires duplicate detection.
        pub requires_duplicate_detection: pulumi_gestalt_rust::Output<bool>,
        /// Boolean flag which controls whether the Queue requires sessions. This will allow ordered handling of unbounded sequences of related messages. With sessions enabled a queue can guarantee first-in-first-out delivery of messages.
        pub requires_session: pulumi_gestalt_rust::Output<bool>,
        pub resource_group_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The status of the Queue. Possible values are `Active`, `Creating`, `Deleting`, `Disabled`, `ReceiveDisabled`, `Renaming`, `SendDisabled`, `Unknown`.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetQueueArgs,
    ) -> GetQueueResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let namespace_id_binding = args.namespace_id.get_output(context);
        let namespace_name_binding = args.namespace_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:servicebus/getQueue:getQueue".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespaceId".into(),
                    value: namespace_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespaceName".into(),
                    value: namespace_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetQueueResult {
            auto_delete_on_idle: o.get_field("autoDeleteOnIdle"),
            dead_lettering_on_message_expiration: o
                .get_field("deadLetteringOnMessageExpiration"),
            default_message_ttl: o.get_field("defaultMessageTtl"),
            duplicate_detection_history_time_window: o
                .get_field("duplicateDetectionHistoryTimeWindow"),
            enable_batched_operations: o.get_field("enableBatchedOperations"),
            enable_express: o.get_field("enableExpress"),
            enable_partitioning: o.get_field("enablePartitioning"),
            forward_dead_lettered_messages_to: o
                .get_field("forwardDeadLetteredMessagesTo"),
            forward_to: o.get_field("forwardTo"),
            id: o.get_field("id"),
            lock_duration: o.get_field("lockDuration"),
            max_delivery_count: o.get_field("maxDeliveryCount"),
            max_size_in_megabytes: o.get_field("maxSizeInMegabytes"),
            name: o.get_field("name"),
            namespace_id: o.get_field("namespaceId"),
            namespace_name: o.get_field("namespaceName"),
            requires_duplicate_detection: o.get_field("requiresDuplicateDetection"),
            requires_session: o.get_field("requiresSession"),
            resource_group_name: o.get_field("resourceGroupName"),
            status: o.get_field("status"),
        }
    }
}
