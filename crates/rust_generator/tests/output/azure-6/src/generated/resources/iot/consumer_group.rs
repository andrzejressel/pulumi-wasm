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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod consumer_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConsumerGroupArgs {
        /// The name of the Event Hub-compatible endpoint in the IoT hub. Changing this forces a new resource to be created.
        #[builder(into)]
        pub eventhub_endpoint_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the IoT Hub. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iothub_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of this Consumer Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group that contains the IoT hub. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ConsumerGroupResult {
        /// The name of the Event Hub-compatible endpoint in the IoT hub. Changing this forces a new resource to be created.
        pub eventhub_endpoint_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the IoT Hub. Changing this forces a new resource to be created.
        pub iothub_name: pulumi_gestalt_rust::Output<String>,
        /// The name of this Consumer Group. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group that contains the IoT hub. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConsumerGroupArgs,
    ) -> ConsumerGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let eventhub_endpoint_name_binding = args
            .eventhub_endpoint_name
            .get_output(context);
        let iothub_name_binding = args.iothub_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:iot/consumerGroup:ConsumerGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhubEndpointName".into(),
                    value: eventhub_endpoint_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iothubName".into(),
                    value: iothub_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConsumerGroupResult {
            eventhub_endpoint_name: o.get_field("eventhubEndpointName"),
            iothub_name: o.get_field("iothubName"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
