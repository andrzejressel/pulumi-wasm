/// Manages a Stream Analytics Output to a ServiceBus Topic.
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
///   exampleNamespace:
///     type: azure:servicebus:Namespace
///     name: example
///     properties:
///       name: example-namespace
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       sku: Standard
///   exampleTopic:
///     type: azure:servicebus:Topic
///     name: example
///     properties:
///       name: example-topic
///       namespaceId: ${exampleNamespace.id}
///       enablePartitioning: true
///   exampleOutputServicebusTopic:
///     type: azure:streamanalytics:OutputServicebusTopic
///     name: example
///     properties:
///       name: service-bus-topic-output
///       streamAnalyticsJobName: ${example.name}
///       resourceGroupName: ${example.resourceGroupName}
///       topicName: ${exampleTopic.name}
///       servicebusNamespace: ${exampleNamespace.name}
///       sharedAccessPolicyKey: ${exampleNamespace.defaultPrimaryKey}
///       sharedAccessPolicyName: RootManageSharedAccessKey
///       propertyColumns:
///         - col1
///         - col2
///       serialization:
///         type: Csv
///         format: Array
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
/// Stream Analytics Output ServiceBus Topic's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/outputServicebusTopic:OutputServicebusTopic example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StreamAnalytics/streamingJobs/job1/outputs/output1
/// ```
///
pub mod output_servicebus_topic {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OutputServicebusTopicArgs {
        /// The authentication mode for the Stream Output. Possible values are `Msi` and `ConnectionString`. Defaults to `ConnectionString`.
        #[builder(into, default)]
        pub authentication_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of property columns to add to the Service Bus Topic output.
        #[builder(into, default)]
        pub property_columns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `serialization` block as defined below.
        #[builder(into)]
        pub serialization: pulumi_wasm_rust::Output<
            super::super::types::streamanalytics::OutputServicebusTopicSerialization,
        >,
        /// The namespace that is associated with the desired Event Hub, Service Bus Topic, Service Bus Topic, etc.
        #[builder(into)]
        pub servicebus_namespace: pulumi_wasm_rust::Output<String>,
        /// The shared access policy key for the specified shared access policy. Required if `authentication_mode` is `ConnectionString`.
        #[builder(into, default)]
        pub shared_access_policy_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The shared access policy name for the Event Hub, Service Bus Queue, Service Bus Topic, etc. Required if `authentication_mode` is `ConnectionString`.
        #[builder(into, default)]
        pub shared_access_policy_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_job_name: pulumi_wasm_rust::Output<String>,
        /// A key-value pair of system property columns that will be attached to the outgoing messages for the Service Bus Topic Output.
        ///
        /// > **NOTE:** The acceptable keys are `ContentType`, `CorrelationId`, `Label`, `MessageId`, `PartitionKey`, `ReplyTo`, `ReplyToSessionId`, `ScheduledEnqueueTimeUtc`, `SessionId`, `TimeToLive` and `To`.
        #[builder(into, default)]
        pub system_property_columns: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Service Bus Topic.
        #[builder(into)]
        pub topic_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct OutputServicebusTopicResult {
        /// The authentication mode for the Stream Output. Possible values are `Msi` and `ConnectionString`. Defaults to `ConnectionString`.
        pub authentication_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of property columns to add to the Service Bus Topic output.
        pub property_columns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `serialization` block as defined below.
        pub serialization: pulumi_wasm_rust::Output<
            super::super::types::streamanalytics::OutputServicebusTopicSerialization,
        >,
        /// The namespace that is associated with the desired Event Hub, Service Bus Topic, Service Bus Topic, etc.
        pub servicebus_namespace: pulumi_wasm_rust::Output<String>,
        /// The shared access policy key for the specified shared access policy. Required if `authentication_mode` is `ConnectionString`.
        pub shared_access_policy_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The shared access policy name for the Event Hub, Service Bus Queue, Service Bus Topic, etc. Required if `authentication_mode` is `ConnectionString`.
        pub shared_access_policy_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        pub stream_analytics_job_name: pulumi_wasm_rust::Output<String>,
        /// A key-value pair of system property columns that will be attached to the outgoing messages for the Service Bus Topic Output.
        ///
        /// > **NOTE:** The acceptable keys are `ContentType`, `CorrelationId`, `Label`, `MessageId`, `PartitionKey`, `ReplyTo`, `ReplyToSessionId`, `ScheduledEnqueueTimeUtc`, `SessionId`, `TimeToLive` and `To`.
        pub system_property_columns: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Service Bus Topic.
        pub topic_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: OutputServicebusTopicArgs,
    ) -> OutputServicebusTopicResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authentication_mode_binding = args.authentication_mode.get_inner();
        let name_binding = args.name.get_inner();
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
        let system_property_columns_binding = args.system_property_columns.get_inner();
        let topic_name_binding = args.topic_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:streamanalytics/outputServicebusTopic:OutputServicebusTopic"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authenticationMode".into(),
                    value: &authentication_mode_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                register_interface::ObjectField {
                    name: "systemPropertyColumns".into(),
                    value: &system_property_columns_binding,
                },
                register_interface::ObjectField {
                    name: "topicName".into(),
                    value: &topic_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authenticationMode".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
                register_interface::ResultField {
                    name: "systemPropertyColumns".into(),
                },
                register_interface::ResultField {
                    name: "topicName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OutputServicebusTopicResult {
            authentication_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationMode").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
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
            system_property_columns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("systemPropertyColumns").unwrap(),
            ),
            topic_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("topicName").unwrap(),
            ),
        }
    }
}