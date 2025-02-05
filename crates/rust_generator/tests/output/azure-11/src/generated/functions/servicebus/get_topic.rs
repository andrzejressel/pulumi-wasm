pub mod get_topic {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTopicArgs {
        /// The name of this Service Bus Topic.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the ServiceBus Namespace where the Service Bus Topic exists.
        #[builder(into, default)]
        pub namespace_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub namespace_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetTopicArgs,
    ) -> GetTopicResult {
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetTopicResult {
            auto_delete_on_idle: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoDeleteOnIdle"),
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
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
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
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            support_ordering: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("supportOrdering"),
            ),
        }
    }
}
