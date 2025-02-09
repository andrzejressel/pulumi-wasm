/// Manages a Stream Analytics Stream Input IoTHub.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleIoTHub:
///     type: azure:iot:IoTHub
///     name: example
///     properties:
///       name: example-iothub
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       sku:
///         name: S1
///         capacity: '1'
///   exampleStreamInputIotHub:
///     type: azure:streamanalytics:StreamInputIotHub
///     name: example
///     properties:
///       name: example-iothub-input
///       streamAnalyticsJobName: ${example.name}
///       resourceGroupName: ${example.resourceGroupName}
///       endpoint: messages/events
///       eventhubConsumerGroupName: $Default
///       iothubNamespace: ${exampleIoTHub.name}
///       sharedAccessPolicyKey: ${exampleIoTHub.sharedAccessPolicies[0].primaryKey}
///       sharedAccessPolicyName: iothubowner
///       serialization:
///         type: Json
///         encoding: UTF8
/// variables:
///   example:
///     fn::invoke:
///       function: azure:streamanalytics:getJob
///       arguments:
///         name: example-job
///         resourceGroupName: ${exampleResourceGroup.name}
/// ```
///
/// ## Import
///
/// Stream Analytics Stream Input IoTHub's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/streamInputIotHub:StreamInputIotHub example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StreamAnalytics/streamingJobs/job1/inputs/input1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod stream_input_iot_hub {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StreamInputIotHubArgs {
        /// The IoT Hub endpoint to connect to (ie. messages/events, messages/operationsMonitoringEvents, etc.).
        #[builder(into)]
        pub endpoint: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of an Event Hub Consumer Group that should be used to read events from the Event Hub. Specifying distinct consumer group names for multiple inputs allows each of those inputs to receive the same events from the Event Hub.
        #[builder(into)]
        pub eventhub_consumer_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name or the URI of the IoT Hub.
        #[builder(into)]
        pub iothub_namespace: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Stream Input IoTHub. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `serialization` block as defined below.
        #[builder(into)]
        pub serialization: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::streamanalytics::StreamInputIotHubSerialization,
        >,
        /// The shared access policy key for the specified shared access policy. Changing this forces a new resource to be created.
        #[builder(into)]
        pub shared_access_policy_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The shared access policy name for the Event Hub, Service Bus Queue, Service Bus Topic, etc.
        #[builder(into)]
        pub shared_access_policy_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_job_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct StreamInputIotHubResult {
        /// The IoT Hub endpoint to connect to (ie. messages/events, messages/operationsMonitoringEvents, etc.).
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// The name of an Event Hub Consumer Group that should be used to read events from the Event Hub. Specifying distinct consumer group names for multiple inputs allows each of those inputs to receive the same events from the Event Hub.
        pub eventhub_consumer_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name or the URI of the IoT Hub.
        pub iothub_namespace: pulumi_gestalt_rust::Output<String>,
        /// The name of the Stream Input IoTHub. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `serialization` block as defined below.
        pub serialization: pulumi_gestalt_rust::Output<
            super::super::types::streamanalytics::StreamInputIotHubSerialization,
        >,
        /// The shared access policy key for the specified shared access policy. Changing this forces a new resource to be created.
        pub shared_access_policy_key: pulumi_gestalt_rust::Output<String>,
        /// The shared access policy name for the Event Hub, Service Bus Queue, Service Bus Topic, etc.
        pub shared_access_policy_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        pub stream_analytics_job_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StreamInputIotHubArgs,
    ) -> StreamInputIotHubResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let endpoint_binding = args.endpoint.get_output(context);
        let eventhub_consumer_group_name_binding = args
            .eventhub_consumer_group_name
            .get_output(context);
        let iothub_namespace_binding = args.iothub_namespace.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let serialization_binding = args.serialization.get_output(context);
        let shared_access_policy_key_binding = args
            .shared_access_policy_key
            .get_output(context);
        let shared_access_policy_name_binding = args
            .shared_access_policy_name
            .get_output(context);
        let stream_analytics_job_name_binding = args
            .stream_analytics_job_name
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:streamanalytics/streamInputIotHub:StreamInputIotHub".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpoint".into(),
                    value: endpoint_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhubConsumerGroupName".into(),
                    value: eventhub_consumer_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iothubNamespace".into(),
                    value: iothub_namespace_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serialization".into(),
                    value: serialization_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sharedAccessPolicyKey".into(),
                    value: shared_access_policy_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sharedAccessPolicyName".into(),
                    value: shared_access_policy_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "streamAnalyticsJobName".into(),
                    value: stream_analytics_job_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        StreamInputIotHubResult {
            endpoint: o.get_field("endpoint"),
            eventhub_consumer_group_name: o.get_field("eventhubConsumerGroupName"),
            iothub_namespace: o.get_field("iothubNamespace"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            serialization: o.get_field("serialization"),
            shared_access_policy_key: o.get_field("sharedAccessPolicyKey"),
            shared_access_policy_name: o.get_field("sharedAccessPolicyName"),
            stream_analytics_job_name: o.get_field("streamAnalyticsJobName"),
        }
    }
}
