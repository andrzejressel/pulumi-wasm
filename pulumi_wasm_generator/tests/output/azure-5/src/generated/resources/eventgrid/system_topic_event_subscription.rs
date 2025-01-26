/// Manages an EventGrid System Topic Event Subscription.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestorageaccount
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///       tags:
///         environment: staging
///   exampleQueue:
///     type: azure:storage:Queue
///     name: example
///     properties:
///       name: examplestoragequeue
///       storageAccountName: ${exampleAccount.name}
///   exampleSystemTopic:
///     type: azure:eventgrid:SystemTopic
///     name: example
///     properties:
///       name: example-system-topic
///       location: Global
///       resourceGroupName: ${example.name}
///       sourceArmResourceId: ${example.id}
///       topicType: Microsoft.Resources.ResourceGroups
///   exampleSystemTopicEventSubscription:
///     type: azure:eventgrid:SystemTopicEventSubscription
///     name: example
///     properties:
///       name: example-event-subscription
///       systemTopic: ${exampleSystemTopic.name}
///       resourceGroupName: ${example.name}
///       storageQueueEndpoint:
///         storageAccountId: ${exampleAccount.id}
///         queueName: ${exampleQueue.name}
/// ```
///
/// ## Import
///
/// EventGrid System Topic Event Subscriptions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventgrid/systemTopicEventSubscription:SystemTopicEventSubscription example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventGrid/systemTopics/topic1/eventSubscriptions/subscription1
/// ```
///
pub mod system_topic_event_subscription {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SystemTopicEventSubscriptionArgs {
        /// A `advanced_filter` block as defined below.
        #[builder(into, default)]
        pub advanced_filter: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilter,
            >,
        >,
        /// Specifies whether advanced filters should be evaluated against an array of values instead of expecting a singular value. Defaults to `false`.
        #[builder(into, default)]
        pub advanced_filtering_on_arrays_enabled: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// An `azure_function_endpoint` block as defined below.
        #[builder(into, default)]
        pub azure_function_endpoint: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionAzureFunctionEndpoint,
            >,
        >,
        /// A `dead_letter_identity` block as defined below.
        ///
        /// > **Note:** `storage_blob_dead_letter_destination` must be specified when a `dead_letter_identity` is specified
        #[builder(into, default)]
        pub dead_letter_identity: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionDeadLetterIdentity,
            >,
        >,
        /// A `delivery_identity` block as defined below.
        #[builder(into, default)]
        pub delivery_identity: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionDeliveryIdentity,
            >,
        >,
        /// One or more `delivery_property` blocks as defined below.
        #[builder(into, default)]
        pub delivery_properties: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::eventgrid::SystemTopicEventSubscriptionDeliveryProperty,
                >,
            >,
        >,
        /// Specifies the event delivery schema for the event subscription. Possible values include: `EventGridSchema`, `CloudEventSchemaV1_0`, `CustomInputSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub event_delivery_schema: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the id where the Event Hub is located.
        #[builder(into, default)]
        pub eventhub_endpoint_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the expiration time of the event subscription (Datetime Format `RFC 3339`).
        #[builder(into, default)]
        pub expiration_time_utc: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the id where the Hybrid Connection is located.
        #[builder(into, default)]
        pub hybrid_connection_endpoint_id: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// A list of applicable event types that need to be part of the event subscription.
        #[builder(into, default)]
        pub included_event_types: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// A list of labels to assign to the event subscription.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name which should be used for this Event Subscription. Changing this forces a new Event Subscription to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the System Topic exists. Changing this forces a new Event Subscription to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A `retry_policy` block as defined below.
        #[builder(into, default)]
        pub retry_policy: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionRetryPolicy,
            >,
        >,
        /// Specifies the id where the Service Bus Queue is located.
        #[builder(into, default)]
        pub service_bus_queue_endpoint_id: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the id where the Service Bus Topic is located.
        #[builder(into, default)]
        pub service_bus_topic_endpoint_id: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// A `storage_blob_dead_letter_destination` block as defined below.
        #[builder(into, default)]
        pub storage_blob_dead_letter_destination: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionStorageBlobDeadLetterDestination,
            >,
        >,
        /// A `storage_queue_endpoint` block as defined below.
        #[builder(into, default)]
        pub storage_queue_endpoint: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionStorageQueueEndpoint,
            >,
        >,
        /// A `subject_filter` block as defined below.
        #[builder(into, default)]
        pub subject_filter: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionSubjectFilter,
            >,
        >,
        /// The System Topic where the Event Subscription should be created in. Changing this forces a new Event Subscription to be created.
        #[builder(into)]
        pub system_topic: pulumi_wasm_rust::InputOrOutput<String>,
        /// A `webhook_endpoint` block as defined below.
        ///
        /// > **NOTE:** One of `azure_function_endpoint`, `eventhub_endpoint_id`, `hybrid_connection_endpoint`, `hybrid_connection_endpoint_id`, `service_bus_queue_endpoint_id`, `service_bus_topic_endpoint_id`, `storage_queue_endpoint` or `webhook_endpoint` must be specified.
        #[builder(into, default)]
        pub webhook_endpoint: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionWebhookEndpoint,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct SystemTopicEventSubscriptionResult {
        /// A `advanced_filter` block as defined below.
        pub advanced_filter: pulumi_wasm_rust::Output<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilter,
            >,
        >,
        /// Specifies whether advanced filters should be evaluated against an array of values instead of expecting a singular value. Defaults to `false`.
        pub advanced_filtering_on_arrays_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `azure_function_endpoint` block as defined below.
        pub azure_function_endpoint: pulumi_wasm_rust::Output<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionAzureFunctionEndpoint,
            >,
        >,
        /// A `dead_letter_identity` block as defined below.
        ///
        /// > **Note:** `storage_blob_dead_letter_destination` must be specified when a `dead_letter_identity` is specified
        pub dead_letter_identity: pulumi_wasm_rust::Output<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionDeadLetterIdentity,
            >,
        >,
        /// A `delivery_identity` block as defined below.
        pub delivery_identity: pulumi_wasm_rust::Output<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionDeliveryIdentity,
            >,
        >,
        /// One or more `delivery_property` blocks as defined below.
        pub delivery_properties: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::eventgrid::SystemTopicEventSubscriptionDeliveryProperty,
                >,
            >,
        >,
        /// Specifies the event delivery schema for the event subscription. Possible values include: `EventGridSchema`, `CloudEventSchemaV1_0`, `CustomInputSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
        pub event_delivery_schema: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the id where the Event Hub is located.
        pub eventhub_endpoint_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the expiration time of the event subscription (Datetime Format `RFC 3339`).
        pub expiration_time_utc: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the id where the Hybrid Connection is located.
        pub hybrid_connection_endpoint_id: pulumi_wasm_rust::Output<String>,
        /// A list of applicable event types that need to be part of the event subscription.
        pub included_event_types: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of labels to assign to the event subscription.
        pub labels: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name which should be used for this Event Subscription. Changing this forces a new Event Subscription to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the System Topic exists. Changing this forces a new Event Subscription to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `retry_policy` block as defined below.
        pub retry_policy: pulumi_wasm_rust::Output<
            super::super::types::eventgrid::SystemTopicEventSubscriptionRetryPolicy,
        >,
        /// Specifies the id where the Service Bus Queue is located.
        pub service_bus_queue_endpoint_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the id where the Service Bus Topic is located.
        pub service_bus_topic_endpoint_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A `storage_blob_dead_letter_destination` block as defined below.
        pub storage_blob_dead_letter_destination: pulumi_wasm_rust::Output<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionStorageBlobDeadLetterDestination,
            >,
        >,
        /// A `storage_queue_endpoint` block as defined below.
        pub storage_queue_endpoint: pulumi_wasm_rust::Output<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionStorageQueueEndpoint,
            >,
        >,
        /// A `subject_filter` block as defined below.
        pub subject_filter: pulumi_wasm_rust::Output<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionSubjectFilter,
            >,
        >,
        /// The System Topic where the Event Subscription should be created in. Changing this forces a new Event Subscription to be created.
        pub system_topic: pulumi_wasm_rust::Output<String>,
        /// A `webhook_endpoint` block as defined below.
        ///
        /// > **NOTE:** One of `azure_function_endpoint`, `eventhub_endpoint_id`, `hybrid_connection_endpoint`, `hybrid_connection_endpoint_id`, `service_bus_queue_endpoint_id`, `service_bus_topic_endpoint_id`, `storage_queue_endpoint` or `webhook_endpoint` must be specified.
        pub webhook_endpoint: pulumi_wasm_rust::Output<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionWebhookEndpoint,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SystemTopicEventSubscriptionArgs,
    ) -> SystemTopicEventSubscriptionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let advanced_filter_binding = args
            .advanced_filter
            .get_output(context)
            .get_inner();
        let advanced_filtering_on_arrays_enabled_binding = args
            .advanced_filtering_on_arrays_enabled
            .get_output(context)
            .get_inner();
        let azure_function_endpoint_binding = args
            .azure_function_endpoint
            .get_output(context)
            .get_inner();
        let dead_letter_identity_binding = args
            .dead_letter_identity
            .get_output(context)
            .get_inner();
        let delivery_identity_binding = args
            .delivery_identity
            .get_output(context)
            .get_inner();
        let delivery_properties_binding = args
            .delivery_properties
            .get_output(context)
            .get_inner();
        let event_delivery_schema_binding = args
            .event_delivery_schema
            .get_output(context)
            .get_inner();
        let eventhub_endpoint_id_binding = args
            .eventhub_endpoint_id
            .get_output(context)
            .get_inner();
        let expiration_time_utc_binding = args
            .expiration_time_utc
            .get_output(context)
            .get_inner();
        let hybrid_connection_endpoint_id_binding = args
            .hybrid_connection_endpoint_id
            .get_output(context)
            .get_inner();
        let included_event_types_binding = args
            .included_event_types
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let retry_policy_binding = args.retry_policy.get_output(context).get_inner();
        let service_bus_queue_endpoint_id_binding = args
            .service_bus_queue_endpoint_id
            .get_output(context)
            .get_inner();
        let service_bus_topic_endpoint_id_binding = args
            .service_bus_topic_endpoint_id
            .get_output(context)
            .get_inner();
        let storage_blob_dead_letter_destination_binding = args
            .storage_blob_dead_letter_destination
            .get_output(context)
            .get_inner();
        let storage_queue_endpoint_binding = args
            .storage_queue_endpoint
            .get_output(context)
            .get_inner();
        let subject_filter_binding = args.subject_filter.get_output(context).get_inner();
        let system_topic_binding = args.system_topic.get_output(context).get_inner();
        let webhook_endpoint_binding = args
            .webhook_endpoint
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:eventgrid/systemTopicEventSubscription:SystemTopicEventSubscription"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "advancedFilter".into(),
                    value: &advanced_filter_binding,
                },
                register_interface::ObjectField {
                    name: "advancedFilteringOnArraysEnabled".into(),
                    value: &advanced_filtering_on_arrays_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "azureFunctionEndpoint".into(),
                    value: &azure_function_endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "deadLetterIdentity".into(),
                    value: &dead_letter_identity_binding,
                },
                register_interface::ObjectField {
                    name: "deliveryIdentity".into(),
                    value: &delivery_identity_binding,
                },
                register_interface::ObjectField {
                    name: "deliveryProperties".into(),
                    value: &delivery_properties_binding,
                },
                register_interface::ObjectField {
                    name: "eventDeliverySchema".into(),
                    value: &event_delivery_schema_binding,
                },
                register_interface::ObjectField {
                    name: "eventhubEndpointId".into(),
                    value: &eventhub_endpoint_id_binding,
                },
                register_interface::ObjectField {
                    name: "expirationTimeUtc".into(),
                    value: &expiration_time_utc_binding,
                },
                register_interface::ObjectField {
                    name: "hybridConnectionEndpointId".into(),
                    value: &hybrid_connection_endpoint_id_binding,
                },
                register_interface::ObjectField {
                    name: "includedEventTypes".into(),
                    value: &included_event_types_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "retryPolicy".into(),
                    value: &retry_policy_binding,
                },
                register_interface::ObjectField {
                    name: "serviceBusQueueEndpointId".into(),
                    value: &service_bus_queue_endpoint_id_binding,
                },
                register_interface::ObjectField {
                    name: "serviceBusTopicEndpointId".into(),
                    value: &service_bus_topic_endpoint_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageBlobDeadLetterDestination".into(),
                    value: &storage_blob_dead_letter_destination_binding,
                },
                register_interface::ObjectField {
                    name: "storageQueueEndpoint".into(),
                    value: &storage_queue_endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "subjectFilter".into(),
                    value: &subject_filter_binding,
                },
                register_interface::ObjectField {
                    name: "systemTopic".into(),
                    value: &system_topic_binding,
                },
                register_interface::ObjectField {
                    name: "webhookEndpoint".into(),
                    value: &webhook_endpoint_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SystemTopicEventSubscriptionResult {
            advanced_filter: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("advancedFilter"),
            ),
            advanced_filtering_on_arrays_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("advancedFilteringOnArraysEnabled"),
            ),
            azure_function_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("azureFunctionEndpoint"),
            ),
            dead_letter_identity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deadLetterIdentity"),
            ),
            delivery_identity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deliveryIdentity"),
            ),
            delivery_properties: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deliveryProperties"),
            ),
            event_delivery_schema: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("eventDeliverySchema"),
            ),
            eventhub_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("eventhubEndpointId"),
            ),
            expiration_time_utc: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("expirationTimeUtc"),
            ),
            hybrid_connection_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hybridConnectionEndpointId"),
            ),
            included_event_types: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("includedEventTypes"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            retry_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("retryPolicy"),
            ),
            service_bus_queue_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceBusQueueEndpointId"),
            ),
            service_bus_topic_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceBusTopicEndpointId"),
            ),
            storage_blob_dead_letter_destination: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageBlobDeadLetterDestination"),
            ),
            storage_queue_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageQueueEndpoint"),
            ),
            subject_filter: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subjectFilter"),
            ),
            system_topic: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("systemTopic"),
            ),
            webhook_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("webhookEndpoint"),
            ),
        }
    }
}
