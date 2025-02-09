/// > **Note:** This resource creates a Stream Input of type `Microsoft.ServiceBus/EventHub`, to create a Stream Input of type `Microsoft.EventHub/EventHub` please use the resource azurerm_stream_analytics_stream_input_eventhub_v2.
///
/// Manages a Stream Analytics Stream Input EventHub.
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
///   exampleStreamInputEventHub:
///     type: azure:streamanalytics:StreamInputEventHub
///     name: example
///     properties:
///       name: eventhub-stream-input
///       streamAnalyticsJobName: ${example.name}
///       resourceGroupName: ${example.resourceGroupName}
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
/// $ pulumi import azure:streamanalytics/streamInputEventHub:StreamInputEventHub example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StreamAnalytics/streamingJobs/job1/inputs/input1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod stream_input_event_hub {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StreamInputEventHubArgs {
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
        /// The name of the Stream Input EventHub. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The property the input Event Hub has been partitioned by.
        #[builder(into, default)]
        pub partition_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `serialization` block as defined below.
        #[builder(into)]
        pub serialization: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::streamanalytics::StreamInputEventHubSerialization,
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
        pub stream_analytics_job_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct StreamInputEventHubResult {
        /// The authentication mode for the Stream Output. Possible values are `Msi` and `ConnectionString`. Defaults to `ConnectionString`.
        pub authentication_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of an Event Hub Consumer Group that should be used to read events from the Event Hub. Specifying distinct consumer group names for multiple inputs allows each of those inputs to receive the same events from the Event Hub. If not set the input will use the Event Hub's default consumer group.
        pub eventhub_consumer_group_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Event Hub.
        pub eventhub_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Stream Input EventHub. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The property the input Event Hub has been partitioned by.
        pub partition_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `serialization` block as defined below.
        pub serialization: pulumi_gestalt_rust::Output<
            super::super::types::streamanalytics::StreamInputEventHubSerialization,
        >,
        /// The namespace that is associated with the desired Event Hub, Service Bus Queue, Service Bus Topic, etc.
        pub servicebus_namespace: pulumi_gestalt_rust::Output<String>,
        /// The shared access policy key for the specified shared access policy.
        pub shared_access_policy_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The shared access policy name for the Event Hub, Service Bus Queue, Service Bus Topic, etc.
        pub shared_access_policy_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        pub stream_analytics_job_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: StreamInputEventHubArgs,
    ) -> StreamInputEventHubResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let authentication_mode_binding_1 = args.authentication_mode.get_output(context);
        let authentication_mode_binding = authentication_mode_binding_1.get_inner();
        let eventhub_consumer_group_name_binding_1 = args
            .eventhub_consumer_group_name
            .get_output(context);
        let eventhub_consumer_group_name_binding = eventhub_consumer_group_name_binding_1
            .get_inner();
        let eventhub_name_binding_1 = args.eventhub_name.get_output(context);
        let eventhub_name_binding = eventhub_name_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let partition_key_binding_1 = args.partition_key.get_output(context);
        let partition_key_binding = partition_key_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let serialization_binding_1 = args.serialization.get_output(context);
        let serialization_binding = serialization_binding_1.get_inner();
        let servicebus_namespace_binding_1 = args
            .servicebus_namespace
            .get_output(context);
        let servicebus_namespace_binding = servicebus_namespace_binding_1.get_inner();
        let shared_access_policy_key_binding_1 = args
            .shared_access_policy_key
            .get_output(context);
        let shared_access_policy_key_binding = shared_access_policy_key_binding_1
            .get_inner();
        let shared_access_policy_name_binding_1 = args
            .shared_access_policy_name
            .get_output(context);
        let shared_access_policy_name_binding = shared_access_policy_name_binding_1
            .get_inner();
        let stream_analytics_job_name_binding_1 = args
            .stream_analytics_job_name
            .get_output(context);
        let stream_analytics_job_name_binding = stream_analytics_job_name_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:streamanalytics/streamInputEventHub:StreamInputEventHub"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authenticationMode".into(),
                    value: &authentication_mode_binding,
                },
                register_interface::ObjectField {
                    name: "eventhubConsumerGroupName".into(),
                    value: &eventhub_consumer_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "eventhubName".into(),
                    value: &eventhub_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "partitionKey".into(),
                    value: &partition_key_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "serialization".into(),
                    value: &serialization_binding,
                },
                register_interface::ObjectField {
                    name: "servicebusNamespace".into(),
                    value: &servicebus_namespace_binding,
                },
                register_interface::ObjectField {
                    name: "sharedAccessPolicyKey".into(),
                    value: &shared_access_policy_key_binding,
                },
                register_interface::ObjectField {
                    name: "sharedAccessPolicyName".into(),
                    value: &shared_access_policy_name_binding,
                },
                register_interface::ObjectField {
                    name: "streamAnalyticsJobName".into(),
                    value: &stream_analytics_job_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        StreamInputEventHubResult {
            authentication_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authenticationMode"),
            ),
            eventhub_consumer_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventhubConsumerGroupName"),
            ),
            eventhub_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventhubName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            partition_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("partitionKey"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            serialization: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serialization"),
            ),
            servicebus_namespace: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("servicebusNamespace"),
            ),
            shared_access_policy_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sharedAccessPolicyKey"),
            ),
            shared_access_policy_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sharedAccessPolicyName"),
            ),
            stream_analytics_job_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("streamAnalyticsJobName"),
            ),
        }
    }
}
