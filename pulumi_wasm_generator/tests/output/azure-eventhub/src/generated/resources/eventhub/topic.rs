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
pub mod topic {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TopicArgs {
        /// The ISO 8601 timespan duration of the idle interval after which the Topic is automatically deleted, minimum of 5 minutes. Defaults to `P10675199DT2H48M5.4775807S`.
        #[builder(into, default)]
        pub auto_delete_on_idle: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean flag which controls if server-side batched operations are enabled.
        #[builder(into, default)]
        pub batched_operations_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ISO 8601 timespan duration of TTL of messages sent to this topic if no TTL value is set on the message itself. Defaults to `P10675199DT2H48M5.4775807S`.
        #[builder(into, default)]
        pub default_message_ttl: pulumi_wasm_rust::Output<Option<String>>,
        /// The ISO 8601 timespan duration during which duplicates can be detected. Defaults to `PT10M` (10 Minutes).
        #[builder(into, default)]
        pub duplicate_detection_history_time_window: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// Boolean flag which controls whether Express Entities are enabled. An express topic holds a message in memory temporarily before writing it to persistent storage.
        #[builder(into, default)]
        pub express_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Integer value which controls the maximum size of a message allowed on the topic for Premium SKU. For supported values see the "Large messages support" section of [this document](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-premium-messaging#large-messages-support-preview). Defaults to `256`.
        #[builder(into, default)]
        pub max_message_size_in_kilobytes: pulumi_wasm_rust::Output<Option<i32>>,
        /// Integer value which controls the size of memory allocated for the topic. For supported values see the "Queue/topic size" section of [this document](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-quotas). Defaults to `5120`.
        #[builder(into, default)]
        pub max_size_in_megabytes: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the name of the ServiceBus Topic resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the ServiceBus Namespace to create this topic in. Changing this forces a new resource to be created.
        #[builder(into)]
        pub namespace_id: pulumi_wasm_rust::Output<String>,
        /// Boolean flag which controls whether to enable the topic to be partitioned across multiple message brokers. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Partitioning is available at entity creation for all queues and topics in Basic or Standard SKUs. It is not available for the Premium messaging SKU, but any previously existing partitioned entities in Premium namespaces continue to work as expected. Please [see the documentation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-partitioning) for more information.
        #[builder(into, default)]
        pub partitioning_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean flag which controls whether the Topic requires duplicate detection. Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub requires_duplicate_detection: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Status of the Service Bus Topic. Acceptable values are `Active` or `Disabled`. Defaults to `Active`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean flag which controls whether the Topic supports ordering.
        #[builder(into, default)]
        pub support_ordering: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct TopicResult {
        /// The ISO 8601 timespan duration of the idle interval after which the Topic is automatically deleted, minimum of 5 minutes. Defaults to `P10675199DT2H48M5.4775807S`.
        pub auto_delete_on_idle: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean flag which controls if server-side batched operations are enabled.
        pub batched_operations_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ISO 8601 timespan duration of TTL of messages sent to this topic if no TTL value is set on the message itself. Defaults to `P10675199DT2H48M5.4775807S`.
        pub default_message_ttl: pulumi_wasm_rust::Output<Option<String>>,
        /// The ISO 8601 timespan duration during which duplicates can be detected. Defaults to `PT10M` (10 Minutes).
        pub duplicate_detection_history_time_window: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// Boolean flag which controls whether Express Entities are enabled. An express topic holds a message in memory temporarily before writing it to persistent storage.
        pub express_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Integer value which controls the maximum size of a message allowed on the topic for Premium SKU. For supported values see the "Large messages support" section of [this document](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-premium-messaging#large-messages-support-preview). Defaults to `256`.
        pub max_message_size_in_kilobytes: pulumi_wasm_rust::Output<i32>,
        /// Integer value which controls the size of memory allocated for the topic. For supported values see the "Queue/topic size" section of [this document](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-quotas). Defaults to `5120`.
        pub max_size_in_megabytes: pulumi_wasm_rust::Output<i32>,
        /// Specifies the name of the ServiceBus Topic resource. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the ServiceBus Namespace to create this topic in. Changing this forces a new resource to be created.
        pub namespace_id: pulumi_wasm_rust::Output<String>,
        pub namespace_name: pulumi_wasm_rust::Output<String>,
        /// Boolean flag which controls whether to enable the topic to be partitioned across multiple message brokers. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Partitioning is available at entity creation for all queues and topics in Basic or Standard SKUs. It is not available for the Premium messaging SKU, but any previously existing partitioned entities in Premium namespaces continue to work as expected. Please [see the documentation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-partitioning) for more information.
        pub partitioning_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean flag which controls whether the Topic requires duplicate detection. Defaults to `false`. Changing this forces a new resource to be created.
        pub requires_duplicate_detection: pulumi_wasm_rust::Output<Option<bool>>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Status of the Service Bus Topic. Acceptable values are `Active` or `Disabled`. Defaults to `Active`.
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean flag which controls whether the Topic supports ordering.
        pub support_ordering: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TopicArgs) -> TopicResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_delete_on_idle_binding = args.auto_delete_on_idle.get_inner();
        let batched_operations_enabled_binding = args
            .batched_operations_enabled
            .get_inner();
        let default_message_ttl_binding = args.default_message_ttl.get_inner();
        let duplicate_detection_history_time_window_binding = args
            .duplicate_detection_history_time_window
            .get_inner();
        let express_enabled_binding = args.express_enabled.get_inner();
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
        let status_binding = args.status.get_inner();
        let support_ordering_binding = args.support_ordering.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:eventhub/topic:Topic".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoDeleteOnIdle".into(),
                },
                register_interface::ResultField {
                    name: "batchedOperationsEnabled".into(),
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
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "supportOrdering".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TopicResult {
            auto_delete_on_idle: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoDeleteOnIdle").unwrap(),
            ),
            batched_operations_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("batchedOperationsEnabled").unwrap(),
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
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            support_ordering: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportOrdering").unwrap(),
            ),
        }
    }
}