/// Manages a Event Hubs Consumer Group as a nested resource within an Event Hub.
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
///       sku: Basic
///       capacity: 2
///       tags:
///         environment: Production
///   exampleEventHub:
///     type: azure:eventhub:EventHub
///     name: example
///     properties:
///       name: acceptanceTestEventHub
///       namespaceName: ${exampleEventHubNamespace.name}
///       resourceGroupName: ${example.name}
///       partitionCount: 2
///       messageRetention: 2
///   exampleConsumerGroup:
///     type: azure:eventhub:ConsumerGroup
///     name: example
///     properties:
///       name: acceptanceTestEventHubConsumerGroup
///       namespaceName: ${exampleEventHubNamespace.name}
///       eventhubName: ${exampleEventHub.name}
///       resourceGroupName: ${example.name}
///       userMetadata: some-meta-data
/// ```
///
/// ## Import
///
/// EventHub Consumer Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventhub/consumerGroup:ConsumerGroup consumerGroup1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventHub/namespaces/namespace1/eventhubs/eventhub1/consumerGroups/consumerGroup1
/// ```
///
pub mod consumer_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConsumerGroupArgs {
        /// Specifies the name of the EventHub. Changing this forces a new resource to be created.
        #[builder(into)]
        pub eventhub_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the EventHub Consumer Group resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the grandparent EventHub Namespace. Changing this forces a new resource to be created.
        #[builder(into)]
        pub namespace_name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the EventHub Consumer Group's grandparent Namespace exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the user metadata.
        #[builder(into, default)]
        pub user_metadata: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ConsumerGroupResult {
        /// Specifies the name of the EventHub. Changing this forces a new resource to be created.
        pub eventhub_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the EventHub Consumer Group resource. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the grandparent EventHub Namespace. Changing this forces a new resource to be created.
        pub namespace_name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the EventHub Consumer Group's grandparent Namespace exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the user metadata.
        pub user_metadata: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConsumerGroupArgs) -> ConsumerGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let eventhub_name_binding = args.eventhub_name.get_inner();
        let name_binding = args.name.get_inner();
        let namespace_name_binding = args.namespace_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let user_metadata_binding = args.user_metadata.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:eventhub/consumerGroup:ConsumerGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "eventhubName".into(),
                    value: &eventhub_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namespaceName".into(),
                    value: &namespace_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "userMetadata".into(),
                    value: &user_metadata_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "eventhubName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namespaceName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "userMetadata".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConsumerGroupResult {
            eventhub_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventhubName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            namespace_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespaceName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            user_metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userMetadata").unwrap(),
            ),
        }
    }
}
