/// Manages a Event Hubs as a nested resource within a Event Hubs namespace.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleEventHubNamespace:
///     type: azure:eventhub:EventHubNamespace
///     name: example
///     properties:
///       name: acceptanceTestEventHubNamespace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Standard
///       capacity: 1
///       tags:
///         environment: Production
///   exampleEventHub:
///     type: azure:eventhub:EventHub
///     name: example
///     properties:
///       name: acceptanceTestEventHub
///       namespaceId: ${exampleEventHubNamespace.id}
///       partitionCount: 2
///       messageRetention: 1
/// ```
///
/// ## Import
///
/// EventHubs can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventhub/eventHub:EventHub eventhub1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventHub/namespaces/namespace1/eventhubs/eventhub1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod event_hub {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventHubArgs {
        /// A `capture_description` block as defined below.
        #[builder(into, default)]
        pub capture_description: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eventhub::EventHubCaptureDescription>,
        >,
        /// Specifies the number of days to retain the events for this Event Hub.
        ///
        /// > **Note:** When using a dedicated Event Hubs cluster, maximum value of `message_retention` is 90 days. When using a shared parent EventHub Namespace, maximum value is 7 days; or 1 day when using a Basic SKU for the shared parent EventHub Namespace.
        #[builder(into)]
        pub message_retention: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Specifies the name of the EventHub resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the EventHub Namespace. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub namespace_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub namespace_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the current number of shards on the Event Hub.
        ///
        /// > **Note:** `partition_count` cannot be changed unless Eventhub Namespace SKU is `Premium` and cannot be decreased.
        ///
        /// > **Note:** When using a dedicated Event Hubs cluster, maximum value of `partition_count` is 1024. When using a shared parent EventHub Namespace, maximum value is 32.
        #[builder(into)]
        pub partition_count: pulumi_gestalt_rust::InputOrOutput<i32>,
        #[builder(into, default)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the status of the Event Hub resource. Possible values are `Active`, `Disabled` and `SendDisabled`. Defaults to `Active`.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EventHubResult {
        /// A `capture_description` block as defined below.
        pub capture_description: pulumi_gestalt_rust::Output<
            Option<super::super::types::eventhub::EventHubCaptureDescription>,
        >,
        /// Specifies the number of days to retain the events for this Event Hub.
        ///
        /// > **Note:** When using a dedicated Event Hubs cluster, maximum value of `message_retention` is 90 days. When using a shared parent EventHub Namespace, maximum value is 7 days; or 1 day when using a Basic SKU for the shared parent EventHub Namespace.
        pub message_retention: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the name of the EventHub resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the EventHub Namespace. Changing this forces a new resource to be created.
        pub namespace_id: pulumi_gestalt_rust::Output<String>,
        pub namespace_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the current number of shards on the Event Hub.
        ///
        /// > **Note:** `partition_count` cannot be changed unless Eventhub Namespace SKU is `Premium` and cannot be decreased.
        ///
        /// > **Note:** When using a dedicated Event Hubs cluster, maximum value of `partition_count` is 1024. When using a shared parent EventHub Namespace, maximum value is 32.
        pub partition_count: pulumi_gestalt_rust::Output<i32>,
        /// The identifiers for partitions created for Event Hubs.
        pub partition_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the status of the Event Hub resource. Possible values are `Active`, `Disabled` and `SendDisabled`. Defaults to `Active`.
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EventHubArgs,
    ) -> EventHubResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let capture_description_binding = args.capture_description.get_output(context);
        let message_retention_binding = args.message_retention.get_output(context);
        let name_binding = args.name.get_output(context);
        let namespace_id_binding = args.namespace_id.get_output(context);
        let namespace_name_binding = args.namespace_name.get_output(context);
        let partition_count_binding = args.partition_count.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let status_binding = args.status.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:eventhub/eventHub:EventHub".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "captureDescription".into(),
                    value: capture_description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "messageRetention".into(),
                    value: message_retention_binding.get_id(),
                },
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
                    name: "partitionCount".into(),
                    value: partition_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: status_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EventHubResult {
            capture_description: o.get_field("captureDescription"),
            message_retention: o.get_field("messageRetention"),
            name: o.get_field("name"),
            namespace_id: o.get_field("namespaceId"),
            namespace_name: o.get_field("namespaceName"),
            partition_count: o.get_field("partitionCount"),
            partition_ids: o.get_field("partitionIds"),
            resource_group_name: o.get_field("resourceGroupName"),
            status: o.get_field("status"),
        }
    }
}
