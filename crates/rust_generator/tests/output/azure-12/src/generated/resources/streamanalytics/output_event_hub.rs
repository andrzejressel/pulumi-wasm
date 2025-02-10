/// Manages a Stream Analytics Output to an EventHub.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: rg-example
///       location: West Europe
///   exampleEventHubNamespace:
///     type: azure:eventhub:EventHubNamespace
///     name: example
///     properties:
///       name: example-ehnamespace
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
///   exampleOutputEventHub:
///     type: azure:streamanalytics:OutputEventHub
///     name: example
///     properties:
///       name: output-to-eventhub
///       streamAnalyticsJobName: ${example.name}
///       resourceGroupName: ${example.resourceGroupName}
///       eventhubName: ${exampleEventHub.name}
///       servicebusNamespace: ${exampleEventHubNamespace.name}
///       sharedAccessPolicyKey: ${exampleEventHubNamespace.defaultPrimaryKey}
///       sharedAccessPolicyName: RootManageSharedAccessKey
///       serialization:
///         type: Avro
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
/// Stream Analytics Outputs to an EventHub can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/outputEventHub:OutputEventHub example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StreamAnalytics/streamingJobs/job1/outputs/output1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod output_event_hub {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OutputEventHubArgs {
        /// The authentication mode for the Stream Output. Possible values are `Msi` and `ConnectionString`. Defaults to `ConnectionString`.
        #[builder(into, default)]
        pub authentication_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Event Hub.
        #[builder(into)]
        pub eventhub_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The column that is used for the Event Hub partition key.
        #[builder(into, default)]
        pub partition_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of property columns to add to the Event Hub output.
        #[builder(into, default)]
        pub property_columns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `serialization` block as defined below.
        #[builder(into)]
        pub serialization: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::streamanalytics::OutputEventHubSerialization,
        >,
        /// The namespace that is associated with the desired Event Hub, Service Bus Queue, Service Bus Topic, etc.
        #[builder(into)]
        pub servicebus_namespace: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The shared access policy key for the specified shared access policy. Required when `authentication_mode` is set to `ConnectionString`.
        #[builder(into, default)]
        pub shared_access_policy_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The shared access policy name for the Event Hub, Service Bus Queue, Service Bus Topic, etc. Required when `authentication_mode` is set to `ConnectionString`.
        #[builder(into, default)]
        pub shared_access_policy_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_job_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct OutputEventHubResult {
        /// The authentication mode for the Stream Output. Possible values are `Msi` and `ConnectionString`. Defaults to `ConnectionString`.
        pub authentication_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Event Hub.
        pub eventhub_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The column that is used for the Event Hub partition key.
        pub partition_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of property columns to add to the Event Hub output.
        pub property_columns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `serialization` block as defined below.
        pub serialization: pulumi_gestalt_rust::Output<
            super::super::types::streamanalytics::OutputEventHubSerialization,
        >,
        /// The namespace that is associated with the desired Event Hub, Service Bus Queue, Service Bus Topic, etc.
        pub servicebus_namespace: pulumi_gestalt_rust::Output<String>,
        /// The shared access policy key for the specified shared access policy. Required when `authentication_mode` is set to `ConnectionString`.
        pub shared_access_policy_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The shared access policy name for the Event Hub, Service Bus Queue, Service Bus Topic, etc. Required when `authentication_mode` is set to `ConnectionString`.
        pub shared_access_policy_name: pulumi_gestalt_rust::Output<Option<String>>,
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
        args: OutputEventHubArgs,
    ) -> OutputEventHubResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authentication_mode_binding = args.authentication_mode.get_output(context);
        let eventhub_name_binding = args.eventhub_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let partition_key_binding = args.partition_key.get_output(context);
        let property_columns_binding = args.property_columns.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let serialization_binding = args.serialization.get_output(context);
        let servicebus_namespace_binding = args.servicebus_namespace.get_output(context);
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
            type_: "azure:streamanalytics/outputEventHub:OutputEventHub".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationMode".into(),
                    value: authentication_mode_binding.get_id(),
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
                    name: "propertyColumns".into(),
                    value: property_columns_binding.get_id(),
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
                    name: "streamAnalyticsJobName".into(),
                    value: stream_analytics_job_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        OutputEventHubResult {
            authentication_mode: o.get_field("authenticationMode"),
            eventhub_name: o.get_field("eventhubName"),
            name: o.get_field("name"),
            partition_key: o.get_field("partitionKey"),
            property_columns: o.get_field("propertyColumns"),
            resource_group_name: o.get_field("resourceGroupName"),
            serialization: o.get_field("serialization"),
            servicebus_namespace: o.get_field("servicebusNamespace"),
            shared_access_policy_key: o.get_field("sharedAccessPolicyKey"),
            shared_access_policy_name: o.get_field("sharedAccessPolicyName"),
            stream_analytics_job_name: o.get_field("streamAnalyticsJobName"),
        }
    }
}
