/// Manages a Consumer Group within an IotHub
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
///   exampleIoTHub:
///     type: azure:iot:IoTHub
///     name: example
///     properties:
///       name: test
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku:
///         name: S1
///         capacity: '1'
///       tags:
///         purpose: testing
///   exampleConsumerGroup:
///     type: azure:iot:ConsumerGroup
///     name: example
///     properties:
///       name: group
///       iothubName: ${exampleIoTHub.name}
///       eventhubEndpointName: events
///       resourceGroupName: ${example.name}
/// ```
///
/// ## Import
///
/// IoTHub Consumer Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:iot/consumerGroup:ConsumerGroup group1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Devices/iotHubs/hub1/eventHubEndpoints/events/consumerGroups/group1
/// ```
///
pub mod consumer_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConsumerGroupArgs {
        /// The name of the Event Hub-compatible endpoint in the IoT hub. Changing this forces a new resource to be created.
        #[builder(into)]
        pub eventhub_endpoint_name: pulumi_wasm_rust::Output<String>,
        /// The name of the IoT Hub. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iothub_name: pulumi_wasm_rust::Output<String>,
        /// The name of this Consumer Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group that contains the IoT hub. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ConsumerGroupResult {
        /// The name of the Event Hub-compatible endpoint in the IoT hub. Changing this forces a new resource to be created.
        pub eventhub_endpoint_name: pulumi_wasm_rust::Output<String>,
        /// The name of the IoT Hub. Changing this forces a new resource to be created.
        pub iothub_name: pulumi_wasm_rust::Output<String>,
        /// The name of this Consumer Group. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group that contains the IoT hub. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConsumerGroupArgs) -> ConsumerGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let eventhub_endpoint_name_binding = args.eventhub_endpoint_name.get_inner();
        let iothub_name_binding = args.iothub_name.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:iot/consumerGroup:ConsumerGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "eventhubEndpointName".into(),
                    value: &eventhub_endpoint_name_binding,
                },
                register_interface::ObjectField {
                    name: "iothubName".into(),
                    value: &iothub_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "eventhubEndpointName".into(),
                },
                register_interface::ResultField {
                    name: "iothubName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
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
            eventhub_endpoint_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventhubEndpointName").unwrap(),
            ),
            iothub_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iothubName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}