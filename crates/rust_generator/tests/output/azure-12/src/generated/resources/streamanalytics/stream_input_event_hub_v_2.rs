/// > **Note:** This resource creates a Stream Input of type `Microsoft.EventHub/EventHub`, to create a Stream Input of type `Microsoft.ServiceBus/EventHub` please use the resource azurerm_stream_analytics_stream_input_eventhub.
///
/// Manages a Stream Analytics Stream Input EventHub V2.
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
///   exampleEventHubNamespace:
///     type: azure:eventhub:EventHubNamespace
///     name: example
///     properties:
///       name: example-namespace
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       sku: Standard
///       capacity: 1
///   exampleEventHub:
///     type: azure:eventhub:EventHub
///     name: example
///     properties:
///       name: example-eventhub
///       namespaceName: ${exampleEventHubNamespace.name}
///       resourceGroupName: ${exampleResourceGroup.name}
///       partitionCount: 2
///       messageRetention: 1
///   exampleConsumerGroup:
///     type: azure:eventhub:ConsumerGroup
///     name: example
///     properties:
///       name: example-consumergroup
///       namespaceName: ${exampleEventHubNamespace.name}
///       eventhubName: ${exampleEventHub.name}
///       resourceGroupName: ${exampleResourceGroup.name}
///   exampleStreamInputEventHubV2:
///     type: azure:streamanalytics:StreamInputEventHubV2
///     name: example
///     properties:
///       name: eventhub-stream-input
///       streamAnalyticsJobId: ${example.id}
///       eventhubConsumerGroupName: ${exampleConsumerGroup.name}
///       eventhubName: ${exampleEventHub.name}
///       servicebusNamespace: ${exampleEventHubNamespace.name}
///       sharedAccessPolicyKey: ${exampleEventHubNamespace.defaultPrimaryKey}
///       sharedAccessPolicyName: RootManageSharedAccessKey
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
/// Stream Analytics Stream Input EventHub's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/streamInputEventHubV2:StreamInputEventHubV2 example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StreamAnalytics/streamingJobs/job1/inputs/input1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod stream_input_event_hub_v_2 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StreamInputEventHubV2Args {
        /// The authentication mode for the Stream Output. Possible values are `Msi` and `ConnectionString`. Defaults to `ConnectionString`.
        #[builder(into, default)]
        pub authentication_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of an Event Hub Consumer Group that should be used to read events from the Event Hub. Specifying distinct consumer group names for multiple inputs allows each of those inputs to receive the same events from the Event Hub. If not set the input will use the Event Hub's default consumer group.
        #[builder(into, default)]
        pub eventhub_consumer_group_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the Event Hub.
        #[builder(into)]
        pub eventhub_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Stream Input EventHub V2. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The property the input Event Hub has been partitioned by.
        #[builder(into, default)]
        pub partition_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `serialization` block as defined below.
        #[builder(into)]
        pub serialization: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::streamanalytics::StreamInputEventHubV2Serialization,
        >,
        /// The namespace that is associated with the desired Event Hub, Service Bus Queue, Service Bus Topic, etc.
        #[builder(into)]
        pub servicebus_namespace: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The shared access policy key for the specified shared access policy.
        #[builder(into, default)]
        pub shared_access_policy_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The shared access policy name for the Event Hub, Service Bus Queue, Service Bus Topic, etc.
        #[builder(into, default)]
        pub shared_access_policy_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_job_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct StreamInputEventHubV2Result {
        /// The authentication mode for the Stream Output. Possible values are `Msi` and `ConnectionString`. Defaults to `ConnectionString`.
        pub authentication_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of an Event Hub Consumer Group that should be used to read events from the Event Hub. Specifying distinct consumer group names for multiple inputs allows each of those inputs to receive the same events from the Event Hub. If not set the input will use the Event Hub's default consumer group.
        pub eventhub_consumer_group_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Event Hub.
        pub eventhub_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Stream Input EventHub V2. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The property the input Event Hub has been partitioned by.
        pub partition_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `serialization` block as defined below.
        pub serialization: pulumi_gestalt_rust::Output<
            super::super::types::streamanalytics::StreamInputEventHubV2Serialization,
        >,
        /// The namespace that is associated with the desired Event Hub, Service Bus Queue, Service Bus Topic, etc.
        pub servicebus_namespace: pulumi_gestalt_rust::Output<String>,
        /// The shared access policy key for the specified shared access policy.
        pub shared_access_policy_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The shared access policy name for the Event Hub, Service Bus Queue, Service Bus Topic, etc.
        pub shared_access_policy_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        pub stream_analytics_job_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StreamInputEventHubV2Args,
    ) -> StreamInputEventHubV2Result {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authentication_mode_binding = args.authentication_mode.get_output(context);
        let eventhub_consumer_group_name_binding = args
            .eventhub_consumer_group_name
            .get_output(context);
        let eventhub_name_binding = args.eventhub_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let partition_key_binding = args.partition_key.get_output(context);
        let serialization_binding = args.serialization.get_output(context);
        let servicebus_namespace_binding = args.servicebus_namespace.get_output(context);
        let shared_access_policy_key_binding = args
            .shared_access_policy_key
            .get_output(context);
        let shared_access_policy_name_binding = args
            .shared_access_policy_name
            .get_output(context);
        let stream_analytics_job_id_binding = args
            .stream_analytics_job_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:streamanalytics/streamInputEventHubV2:StreamInputEventHubV2"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationMode".into(),
                    value: authentication_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhubConsumerGroupName".into(),
                    value: eventhub_consumer_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventhubName".into(),
                    value: eventhub_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "partitionKey".into(),
                    value: partition_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serialization".into(),
                    value: serialization_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "servicebusNamespace".into(),
                    value: servicebus_namespace_binding.get_id(),
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
                    name: "streamAnalyticsJobId".into(),
                    value: stream_analytics_job_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        StreamInputEventHubV2Result {
            authentication_mode: o.get_field("authenticationMode"),
            eventhub_consumer_group_name: o.get_field("eventhubConsumerGroupName"),
            eventhub_name: o.get_field("eventhubName"),
            name: o.get_field("name"),
            partition_key: o.get_field("partitionKey"),
            serialization: o.get_field("serialization"),
            servicebus_namespace: o.get_field("servicebusNamespace"),
            shared_access_policy_key: o.get_field("sharedAccessPolicyKey"),
            shared_access_policy_name: o.get_field("sharedAccessPolicyName"),
            stream_analytics_job_id: o.get_field("streamAnalyticsJobId"),
        }
    }
}
