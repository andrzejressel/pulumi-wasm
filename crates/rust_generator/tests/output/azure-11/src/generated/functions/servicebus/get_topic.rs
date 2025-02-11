#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_topic {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTopicArgs {
        /// The name of this Service Bus Topic.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the ServiceBus Namespace where the Service Bus Topic exists.
        #[builder(into, default)]
        pub namespace_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub namespace_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetTopicResult {
        /// The ISO 8601 timespan duration of the idle interval after which the Topic is automatically deleted, minimum of 5 minutes.
        pub auto_delete_on_idle: pulumi_gestalt_rust::Output<String>,
        /// The ISO 8601 timespan duration of TTL of messages sent to this topic if no TTL value is set on the message itself.
        pub default_message_ttl: pulumi_gestalt_rust::Output<String>,
        /// The ISO 8601 timespan duration during which duplicates can be detected.
        pub duplicate_detection_history_time_window: pulumi_gestalt_rust::Output<String>,
        /// Boolean flag which controls if server-side batched operations are enabled.
        pub enable_batched_operations: pulumi_gestalt_rust::Output<bool>,
        /// Boolean flag which controls whether Express Entities are enabled. An express topic holds a message in memory temporarily before writing it to persistent storage.
        pub enable_express: pulumi_gestalt_rust::Output<bool>,
        /// Boolean flag which controls whether to enable the topic to be partitioned across multiple message brokers.
        pub enable_partitioning: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Integer value which controls the size of memory allocated for the topic. For supported values see the "Queue/topic size" section of [this document](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-quotas).
        pub max_size_in_megabytes: pulumi_gestalt_rust::Output<i32>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub namespace_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub namespace_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Boolean flag which controls whether the Topic requires duplicate detection.
        pub requires_duplicate_detection: pulumi_gestalt_rust::Output<bool>,
        pub resource_group_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Status of the Service Bus Topic. Acceptable values are Active or Disabled.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Boolean flag which controls whether the Topic supports ordering.
        pub support_ordering: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetTopicArgs,
    ) -> GetTopicResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let namespace_id_binding = args.namespace_id.get_output(context);
        let namespace_name_binding = args.namespace_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:servicebus/getTopic:getTopic".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespaceId".into(),
                    value: &namespace_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespaceName".into(),
                    value: &namespace_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTopicResult {
            auto_delete_on_idle: o.get_field("autoDeleteOnIdle"),
            default_message_ttl: o.get_field("defaultMessageTtl"),
            duplicate_detection_history_time_window: o
                .get_field("duplicateDetectionHistoryTimeWindow"),
            enable_batched_operations: o.get_field("enableBatchedOperations"),
            enable_express: o.get_field("enableExpress"),
            enable_partitioning: o.get_field("enablePartitioning"),
            id: o.get_field("id"),
            max_size_in_megabytes: o.get_field("maxSizeInMegabytes"),
            name: o.get_field("name"),
            namespace_id: o.get_field("namespaceId"),
            namespace_name: o.get_field("namespaceName"),
            requires_duplicate_detection: o.get_field("requiresDuplicateDetection"),
            resource_group_name: o.get_field("resourceGroupName"),
            status: o.get_field("status"),
            support_ordering: o.get_field("supportOrdering"),
        }
    }
}
