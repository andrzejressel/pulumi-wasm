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
pub mod event_hub {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventHubArgs {
        /// A `capture_description` block as defined below.
        #[builder(into, default)]
        pub capture_description: pulumi_wasm_rust::Output<
            Option<super::types::eventhub::EventHubCaptureDescription>,
        >,
        /// Specifies the number of days to retain the events for this Event Hub.
        ///
        /// > **Note:** When using a dedicated Event Hubs cluster, maximum value of `message_retention` is 90 days. When using a shared parent EventHub Namespace, maximum value is 7 days; or 1 day when using a Basic SKU for the shared parent EventHub Namespace.
        #[builder(into)]
        pub message_retention: pulumi_wasm_rust::Output<i32>,
        /// Specifies the name of the EventHub resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the EventHub Namespace. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub namespace_id: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub namespace_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the current number of shards on the Event Hub.
        ///
        /// > **Note:** `partition_count` cannot be changed unless Eventhub Namespace SKU is `Premium` and cannot be decreased.
        ///
        /// > **Note:** When using a dedicated Event Hubs cluster, maximum value of `partition_count` is 1024. When using a shared parent EventHub Namespace, maximum value is 32.
        #[builder(into)]
        pub partition_count: pulumi_wasm_rust::Output<i32>,
        #[builder(into, default)]
        pub resource_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the status of the Event Hub resource. Possible values are `Active`, `Disabled` and `SendDisabled`. Defaults to `Active`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EventHubResult {
        /// A `capture_description` block as defined below.
        pub capture_description: pulumi_wasm_rust::Output<
            Option<super::types::eventhub::EventHubCaptureDescription>,
        >,
        /// Specifies the number of days to retain the events for this Event Hub.
        ///
        /// > **Note:** When using a dedicated Event Hubs cluster, maximum value of `message_retention` is 90 days. When using a shared parent EventHub Namespace, maximum value is 7 days; or 1 day when using a Basic SKU for the shared parent EventHub Namespace.
        pub message_retention: pulumi_wasm_rust::Output<i32>,
        /// Specifies the name of the EventHub resource. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the EventHub Namespace. Changing this forces a new resource to be created.
        pub namespace_id: pulumi_wasm_rust::Output<String>,
        pub namespace_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the current number of shards on the Event Hub.
        ///
        /// > **Note:** `partition_count` cannot be changed unless Eventhub Namespace SKU is `Premium` and cannot be decreased.
        ///
        /// > **Note:** When using a dedicated Event Hubs cluster, maximum value of `partition_count` is 1024. When using a shared parent EventHub Namespace, maximum value is 32.
        pub partition_count: pulumi_wasm_rust::Output<i32>,
        /// The identifiers for partitions created for Event Hubs.
        pub partition_ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the status of the Event Hub resource. Possible values are `Active`, `Disabled` and `SendDisabled`. Defaults to `Active`.
        pub status: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EventHubArgs) -> EventHubResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let capture_description_binding = args.capture_description.get_inner();
        let message_retention_binding = args.message_retention.get_inner();
        let name_binding = args.name.get_inner();
        let namespace_id_binding = args.namespace_id.get_inner();
        let namespace_name_binding = args.namespace_name.get_inner();
        let partition_count_binding = args.partition_count.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let status_binding = args.status.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:eventhub/eventHub:EventHub".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "captureDescription".into(),
                    value: &capture_description_binding,
                },
                register_interface::ObjectField {
                    name: "messageRetention".into(),
                    value: &message_retention_binding,
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
                    name: "namespaceName".into(),
                    value: &namespace_name_binding,
                },
                register_interface::ObjectField {
                    name: "partitionCount".into(),
                    value: &partition_count_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "captureDescription".into(),
                },
                register_interface::ResultField {
                    name: "messageRetention".into(),
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
                    name: "partitionCount".into(),
                },
                register_interface::ResultField {
                    name: "partitionIds".into(),
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
        EventHubResult {
            capture_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("captureDescription").unwrap(),
            ),
            message_retention: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("messageRetention").unwrap(),
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
            partition_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partitionCount").unwrap(),
            ),
            partition_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partitionIds").unwrap(),
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
