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
pub mod output_event_hub {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OutputEventHubArgs {
        /// The authentication mode for the Stream Output. Possible values are `Msi` and `ConnectionString`. Defaults to `ConnectionString`.
        #[builder(into, default)]
        pub authentication_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Event Hub.
        #[builder(into)]
        pub eventhub_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The column that is used for the Event Hub partition key.
        #[builder(into, default)]
        pub partition_key: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of property columns to add to the Event Hub output.
        #[builder(into, default)]
        pub property_columns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `serialization` block as defined below.
        #[builder(into)]
        pub serialization: pulumi_wasm_rust::Output<
            super::super::types::streamanalytics::OutputEventHubSerialization,
        >,
        /// The namespace that is associated with the desired Event Hub, Service Bus Queue, Service Bus Topic, etc.
        #[builder(into)]
        pub servicebus_namespace: pulumi_wasm_rust::Output<String>,
        /// The shared access policy key for the specified shared access policy. Required when `authentication_mode` is set to `ConnectionString`.
        #[builder(into, default)]
        pub shared_access_policy_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The shared access policy name for the Event Hub, Service Bus Queue, Service Bus Topic, etc. Required when `authentication_mode` is set to `ConnectionString`.
        #[builder(into, default)]
        pub shared_access_policy_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_job_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct OutputEventHubResult {
        /// The authentication mode for the Stream Output. Possible values are `Msi` and `ConnectionString`. Defaults to `ConnectionString`.
        pub authentication_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Event Hub.
        pub eventhub_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The column that is used for the Event Hub partition key.
        pub partition_key: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of property columns to add to the Event Hub output.
        pub property_columns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `serialization` block as defined below.
        pub serialization: pulumi_wasm_rust::Output<
            super::super::types::streamanalytics::OutputEventHubSerialization,
        >,
        /// The namespace that is associated with the desired Event Hub, Service Bus Queue, Service Bus Topic, etc.
        pub servicebus_namespace: pulumi_wasm_rust::Output<String>,
        /// The shared access policy key for the specified shared access policy. Required when `authentication_mode` is set to `ConnectionString`.
        pub shared_access_policy_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The shared access policy name for the Event Hub, Service Bus Queue, Service Bus Topic, etc. Required when `authentication_mode` is set to `ConnectionString`.
        pub shared_access_policy_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        pub stream_analytics_job_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: OutputEventHubArgs) -> OutputEventHubResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authentication_mode_binding = args.authentication_mode.get_inner();
        let eventhub_name_binding = args.eventhub_name.get_inner();
        let name_binding = args.name.get_inner();
        let partition_key_binding = args.partition_key.get_inner();
        let property_columns_binding = args.property_columns.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let serialization_binding = args.serialization.get_inner();
        let servicebus_namespace_binding = args.servicebus_namespace.get_inner();
        let shared_access_policy_key_binding = args.shared_access_policy_key.get_inner();
        let shared_access_policy_name_binding = args
            .shared_access_policy_name
            .get_inner();
        let stream_analytics_job_name_binding = args
            .stream_analytics_job_name
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:streamanalytics/outputEventHub:OutputEventHub".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authenticationMode".into(),
                    value: &authentication_mode_binding,
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
                    name: "propertyColumns".into(),
                    value: &property_columns_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "authenticationMode".into(),
                },
                register_interface::ResultField {
                    name: "eventhubName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "partitionKey".into(),
                },
                register_interface::ResultField {
                    name: "propertyColumns".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "serialization".into(),
                },
                register_interface::ResultField {
                    name: "servicebusNamespace".into(),
                },
                register_interface::ResultField {
                    name: "sharedAccessPolicyKey".into(),
                },
                register_interface::ResultField {
                    name: "sharedAccessPolicyName".into(),
                },
                register_interface::ResultField {
                    name: "streamAnalyticsJobName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OutputEventHubResult {
            authentication_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationMode").unwrap(),
            ),
            eventhub_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventhubName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            partition_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partitionKey").unwrap(),
            ),
            property_columns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("propertyColumns").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            serialization: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serialization").unwrap(),
            ),
            servicebus_namespace: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("servicebusNamespace").unwrap(),
            ),
            shared_access_policy_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sharedAccessPolicyKey").unwrap(),
            ),
            shared_access_policy_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sharedAccessPolicyName").unwrap(),
            ),
            stream_analytics_job_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamAnalyticsJobName").unwrap(),
            ),
        }
    }
}
