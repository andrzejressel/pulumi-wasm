pub mod get_topic {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTopicArgs {
        /// The name of this Service Bus Topic.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the ServiceBus Namespace where the Service Bus Topic exists.
        #[builder(into, default)]
        pub namespace_id: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub namespace_name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub resource_group_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetTopicResult {
        /// The ISO 8601 timespan duration of the idle interval after which the Topic is automatically deleted, minimum of 5 minutes.
        pub auto_delete_on_idle: pulumi_wasm_rust::Output<String>,
        /// The ISO 8601 timespan duration of TTL of messages sent to this topic if no TTL value is set on the message itself.
        pub default_message_ttl: pulumi_wasm_rust::Output<String>,
        /// The ISO 8601 timespan duration during which duplicates can be detected.
        pub duplicate_detection_history_time_window: pulumi_wasm_rust::Output<String>,
        /// Boolean flag which controls if server-side batched operations are enabled.
        pub enable_batched_operations: pulumi_wasm_rust::Output<bool>,
        /// Boolean flag which controls whether Express Entities are enabled. An express topic holds a message in memory temporarily before writing it to persistent storage.
        pub enable_express: pulumi_wasm_rust::Output<bool>,
        /// Boolean flag which controls whether to enable the topic to be partitioned across multiple message brokers.
        pub enable_partitioning: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Integer value which controls the size of memory allocated for the topic. For supported values see the "Queue/topic size" section of [this document](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-quotas).
        pub max_size_in_megabytes: pulumi_wasm_rust::Output<i32>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub namespace_id: pulumi_wasm_rust::Output<Option<String>>,
        pub namespace_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean flag which controls whether the Topic requires duplicate detection.
        pub requires_duplicate_detection: pulumi_wasm_rust::Output<bool>,
        pub resource_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Status of the Service Bus Topic. Acceptable values are Active or Disabled.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Boolean flag which controls whether the Topic supports ordering.
        pub support_ordering: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetTopicArgs) -> GetTopicResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let namespace_id_binding = args.namespace_id.get_inner();
        let namespace_name_binding = args.namespace_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:servicebus/getTopic:getTopic".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoDeleteOnIdle".into(),
                },
                register_interface::ResultField {
                    name: "defaultMessageTtl".into(),
                },
                register_interface::ResultField {
                    name: "duplicateDetectionHistoryTimeWindow".into(),
                },
                register_interface::ResultField {
                    name: "enableBatchedOperations".into(),
                },
                register_interface::ResultField {
                    name: "enableExpress".into(),
                },
                register_interface::ResultField {
                    name: "enablePartitioning".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
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
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetTopicResult {
            auto_delete_on_idle: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoDeleteOnIdle").unwrap(),
            ),
            default_message_ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultMessageTtl").unwrap(),
            ),
            duplicate_detection_history_time_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("duplicateDetectionHistoryTimeWindow").unwrap(),
            ),
            enable_batched_operations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableBatchedOperations").unwrap(),
            ),
            enable_express: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableExpress").unwrap(),
            ),
            enable_partitioning: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enablePartitioning").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
