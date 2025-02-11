/// Manages a ServiceBus Topic.
///
/// **Note** Topics can only be created in Namespaces with an SKU of `standard` or higher.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: tfex-servicebus-topic
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
///       partitioningEnabled: true
/// ```
///
/// ## Import
///
/// Service Bus Topics can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventhub/topic:Topic example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ServiceBus/namespaces/sbns1/topics/sntopic1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod topic {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TopicArgs {
        /// The ISO 8601 timespan duration of the idle interval after which the Topic is automatically deleted, minimum of 5 minutes. Defaults to `P10675199DT2H48M5.4775807S`.
        #[builder(into, default)]
        pub auto_delete_on_idle: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boolean flag which controls if server-side batched operations are enabled.
        #[builder(into, default)]
        pub batched_operations_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ISO 8601 timespan duration of TTL of messages sent to this topic if no TTL value is set on the message itself. Defaults to `P10675199DT2H48M5.4775807S`.
        #[builder(into, default)]
        pub default_message_ttl: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ISO 8601 timespan duration during which duplicates can be detected. Defaults to `PT10M` (10 Minutes).
        #[builder(into, default)]
        pub duplicate_detection_history_time_window: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Boolean flag which controls whether Express Entities are enabled. An express topic holds a message in memory temporarily before writing it to persistent storage.
        #[builder(into, default)]
        pub express_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Integer value which controls the maximum size of a message allowed on the topic for Premium SKU. For supported values see the "Large messages support" section of [this document](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-premium-messaging#large-messages-support-preview). Defaults to `256`.
        #[builder(into, default)]
        pub max_message_size_in_kilobytes: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Integer value which controls the size of memory allocated for the topic. For supported values see the "Queue/topic size" section of [this document](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-quotas). Defaults to `5120`.
        #[builder(into, default)]
        pub max_size_in_megabytes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the name of the ServiceBus Topic resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the ServiceBus Namespace to create this topic in. Changing this forces a new resource to be created.
        #[builder(into)]
        pub namespace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Boolean flag which controls whether to enable the topic to be partitioned across multiple message brokers. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Partitioning is available at entity creation for all queues and topics in Basic or Standard SKUs. It is not available for the Premium messaging SKU, but any previously existing partitioned entities in Premium namespaces continue to work as expected. Please [see the documentation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-partitioning) for more information.
        #[builder(into, default)]
        pub partitioning_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Boolean flag which controls whether the Topic requires duplicate detection. Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub requires_duplicate_detection: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The Status of the Service Bus Topic. Acceptable values are `Active` or `Disabled`. Defaults to `Active`.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boolean flag which controls whether the Topic supports ordering.
        #[builder(into, default)]
        pub support_ordering: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct TopicResult {
        /// The ISO 8601 timespan duration of the idle interval after which the Topic is automatically deleted, minimum of 5 minutes. Defaults to `P10675199DT2H48M5.4775807S`.
        pub auto_delete_on_idle: pulumi_gestalt_rust::Output<Option<String>>,
        /// Boolean flag which controls if server-side batched operations are enabled.
        pub batched_operations_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ISO 8601 timespan duration of TTL of messages sent to this topic if no TTL value is set on the message itself. Defaults to `P10675199DT2H48M5.4775807S`.
        pub default_message_ttl: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ISO 8601 timespan duration during which duplicates can be detected. Defaults to `PT10M` (10 Minutes).
        pub duplicate_detection_history_time_window: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Boolean flag which controls whether Express Entities are enabled. An express topic holds a message in memory temporarily before writing it to persistent storage.
        pub express_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Integer value which controls the maximum size of a message allowed on the topic for Premium SKU. For supported values see the "Large messages support" section of [this document](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-premium-messaging#large-messages-support-preview). Defaults to `256`.
        pub max_message_size_in_kilobytes: pulumi_gestalt_rust::Output<i32>,
        /// Integer value which controls the size of memory allocated for the topic. For supported values see the "Queue/topic size" section of [this document](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-quotas). Defaults to `5120`.
        pub max_size_in_megabytes: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the name of the ServiceBus Topic resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the ServiceBus Namespace to create this topic in. Changing this forces a new resource to be created.
        pub namespace_id: pulumi_gestalt_rust::Output<String>,
        pub namespace_name: pulumi_gestalt_rust::Output<String>,
        /// Boolean flag which controls whether to enable the topic to be partitioned across multiple message brokers. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Partitioning is available at entity creation for all queues and topics in Basic or Standard SKUs. It is not available for the Premium messaging SKU, but any previously existing partitioned entities in Premium namespaces continue to work as expected. Please [see the documentation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-partitioning) for more information.
        pub partitioning_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Boolean flag which controls whether the Topic requires duplicate detection. Defaults to `false`. Changing this forces a new resource to be created.
        pub requires_duplicate_detection: pulumi_gestalt_rust::Output<Option<bool>>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Status of the Service Bus Topic. Acceptable values are `Active` or `Disabled`. Defaults to `Active`.
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
        /// Boolean flag which controls whether the Topic supports ordering.
        pub support_ordering: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TopicArgs,
    ) -> TopicResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_delete_on_idle_binding = args.auto_delete_on_idle.get_output(context);
        let batched_operations_enabled_binding = args
            .batched_operations_enabled
            .get_output(context);
        let default_message_ttl_binding = args.default_message_ttl.get_output(context);
        let duplicate_detection_history_time_window_binding = args
            .duplicate_detection_history_time_window
            .get_output(context);
        let express_enabled_binding = args.express_enabled.get_output(context);
        let max_message_size_in_kilobytes_binding = args
            .max_message_size_in_kilobytes
            .get_output(context);
        let max_size_in_megabytes_binding = args
            .max_size_in_megabytes
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let namespace_id_binding = args.namespace_id.get_output(context);
        let partitioning_enabled_binding = args.partitioning_enabled.get_output(context);
        let requires_duplicate_detection_binding = args
            .requires_duplicate_detection
            .get_output(context);
        let status_binding = args.status.get_output(context);
        let support_ordering_binding = args.support_ordering.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:eventhub/topic:Topic".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoDeleteOnIdle".into(),
                    value: &auto_delete_on_idle_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "batchedOperationsEnabled".into(),
                    value: &batched_operations_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultMessageTtl".into(),
                    value: &default_message_ttl_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "duplicateDetectionHistoryTimeWindow".into(),
                    value: &duplicate_detection_history_time_window_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expressEnabled".into(),
                    value: &express_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxMessageSizeInKilobytes".into(),
                    value: &max_message_size_in_kilobytes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxSizeInMegabytes".into(),
                    value: &max_size_in_megabytes_binding.drop_type(),
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
                    name: "partitioningEnabled".into(),
                    value: &partitioning_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requiresDuplicateDetection".into(),
                    value: &requires_duplicate_detection_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "supportOrdering".into(),
                    value: &support_ordering_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TopicResult {
            auto_delete_on_idle: o.get_field("autoDeleteOnIdle"),
            batched_operations_enabled: o.get_field("batchedOperationsEnabled"),
            default_message_ttl: o.get_field("defaultMessageTtl"),
            duplicate_detection_history_time_window: o
                .get_field("duplicateDetectionHistoryTimeWindow"),
            express_enabled: o.get_field("expressEnabled"),
            max_message_size_in_kilobytes: o.get_field("maxMessageSizeInKilobytes"),
            max_size_in_megabytes: o.get_field("maxSizeInMegabytes"),
            name: o.get_field("name"),
            namespace_id: o.get_field("namespaceId"),
            namespace_name: o.get_field("namespaceName"),
            partitioning_enabled: o.get_field("partitioningEnabled"),
            requires_duplicate_detection: o.get_field("requiresDuplicateDetection"),
            resource_group_name: o.get_field("resourceGroupName"),
            status: o.get_field("status"),
            support_ordering: o.get_field("supportOrdering"),
        }
    }
}
