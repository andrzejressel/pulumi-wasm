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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TopicArgs,
    ) -> TopicResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let auto_delete_on_idle_binding_1 = args.auto_delete_on_idle.get_output(context);
        let auto_delete_on_idle_binding = auto_delete_on_idle_binding_1.get_inner();
        let batched_operations_enabled_binding_1 = args
            .batched_operations_enabled
            .get_output(context);
        let batched_operations_enabled_binding = batched_operations_enabled_binding_1
            .get_inner();
        let default_message_ttl_binding_1 = args.default_message_ttl.get_output(context);
        let default_message_ttl_binding = default_message_ttl_binding_1.get_inner();
        let duplicate_detection_history_time_window_binding_1 = args
            .duplicate_detection_history_time_window
            .get_output(context);
        let duplicate_detection_history_time_window_binding = duplicate_detection_history_time_window_binding_1
            .get_inner();
        let express_enabled_binding_1 = args.express_enabled.get_output(context);
        let express_enabled_binding = express_enabled_binding_1.get_inner();
        let max_message_size_in_kilobytes_binding_1 = args
            .max_message_size_in_kilobytes
            .get_output(context);
        let max_message_size_in_kilobytes_binding = max_message_size_in_kilobytes_binding_1
            .get_inner();
        let max_size_in_megabytes_binding_1 = args
            .max_size_in_megabytes
            .get_output(context);
        let max_size_in_megabytes_binding = max_size_in_megabytes_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let namespace_id_binding_1 = args.namespace_id.get_output(context);
        let namespace_id_binding = namespace_id_binding_1.get_inner();
        let partitioning_enabled_binding_1 = args
            .partitioning_enabled
            .get_output(context);
        let partitioning_enabled_binding = partitioning_enabled_binding_1.get_inner();
        let requires_duplicate_detection_binding_1 = args
            .requires_duplicate_detection
            .get_output(context);
        let requires_duplicate_detection_binding = requires_duplicate_detection_binding_1
            .get_inner();
        let status_binding_1 = args.status.get_output(context);
        let status_binding = status_binding_1.get_inner();
        let support_ordering_binding_1 = args.support_ordering.get_output(context);
        let support_ordering_binding = support_ordering_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:eventhub/topic:Topic".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "supportOrdering".into(),
                    value: &support_ordering_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TopicResult {
            auto_delete_on_idle: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoDeleteOnIdle"),
            ),
            batched_operations_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("batchedOperationsEnabled"),
            ),
            default_message_ttl: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultMessageTtl"),
            ),
            duplicate_detection_history_time_window: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("duplicateDetectionHistoryTimeWindow"),
            ),
            express_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expressEnabled"),
            ),
            max_message_size_in_kilobytes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxMessageSizeInKilobytes"),
            ),
            max_size_in_megabytes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxSizeInMegabytes"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            namespace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namespaceId"),
            ),
            namespace_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namespaceName"),
            ),
            partitioning_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("partitioningEnabled"),
            ),
            requires_duplicate_detection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requiresDuplicateDetection"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            support_ordering: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportOrdering"),
            ),
        }
    }
}
