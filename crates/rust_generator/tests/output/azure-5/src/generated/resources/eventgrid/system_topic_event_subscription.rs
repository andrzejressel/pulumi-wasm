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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod system_topic_event_subscription {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SystemTopicEventSubscriptionArgs {
        /// A `advanced_filter` block as defined below.
        #[builder(into, default)]
        pub advanced_filter: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilter,
            >,
        >,
        /// Specifies whether advanced filters should be evaluated against an array of values instead of expecting a singular value. Defaults to `false`.
        #[builder(into, default)]
        pub advanced_filtering_on_arrays_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// An `azure_function_endpoint` block as defined below.
        #[builder(into, default)]
        pub azure_function_endpoint: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionAzureFunctionEndpoint,
            >,
        >,
        /// A `dead_letter_identity` block as defined below.
        ///
        /// > **Note:** `storage_blob_dead_letter_destination` must be specified when a `dead_letter_identity` is specified
        #[builder(into, default)]
        pub dead_letter_identity: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionDeadLetterIdentity,
            >,
        >,
        /// A `delivery_identity` block as defined below.
        #[builder(into, default)]
        pub delivery_identity: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionDeliveryIdentity,
            >,
        >,
        /// One or more `delivery_property` blocks as defined below.
        #[builder(into, default)]
        pub delivery_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::eventgrid::SystemTopicEventSubscriptionDeliveryProperty,
                >,
            >,
        >,
        /// Specifies the event delivery schema for the event subscription. Possible values include: `EventGridSchema`, `CloudEventSchemaV1_0`, `CustomInputSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub event_delivery_schema: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the id where the Event Hub is located.
        #[builder(into, default)]
        pub eventhub_endpoint_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the expiration time of the event subscription (Datetime Format `RFC 3339`).
        #[builder(into, default)]
        pub expiration_time_utc: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the id where the Hybrid Connection is located.
        #[builder(into, default)]
        pub hybrid_connection_endpoint_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A list of applicable event types that need to be part of the event subscription.
        #[builder(into, default)]
        pub included_event_types: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// A list of labels to assign to the event subscription.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name which should be used for this Event Subscription. Changing this forces a new Event Subscription to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the System Topic exists. Changing this forces a new Event Subscription to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `retry_policy` block as defined below.
        #[builder(into, default)]
        pub retry_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionRetryPolicy,
            >,
        >,
        /// Specifies the id where the Service Bus Queue is located.
        #[builder(into, default)]
        pub service_bus_queue_endpoint_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the id where the Service Bus Topic is located.
        #[builder(into, default)]
        pub service_bus_topic_endpoint_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A `storage_blob_dead_letter_destination` block as defined below.
        #[builder(into, default)]
        pub storage_blob_dead_letter_destination: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionStorageBlobDeadLetterDestination,
            >,
        >,
        /// A `storage_queue_endpoint` block as defined below.
        #[builder(into, default)]
        pub storage_queue_endpoint: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionStorageQueueEndpoint,
            >,
        >,
        /// A `subject_filter` block as defined below.
        #[builder(into, default)]
        pub subject_filter: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionSubjectFilter,
            >,
        >,
        /// The System Topic where the Event Subscription should be created in. Changing this forces a new Event Subscription to be created.
        #[builder(into)]
        pub system_topic: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `webhook_endpoint` block as defined below.
        ///
        /// > **NOTE:** One of `azure_function_endpoint`, `eventhub_endpoint_id`, `hybrid_connection_endpoint`, `hybrid_connection_endpoint_id`, `service_bus_queue_endpoint_id`, `service_bus_topic_endpoint_id`, `storage_queue_endpoint` or `webhook_endpoint` must be specified.
        #[builder(into, default)]
        pub webhook_endpoint: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionWebhookEndpoint,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct SystemTopicEventSubscriptionResult {
        /// A `advanced_filter` block as defined below.
        pub advanced_filter: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionAdvancedFilter,
            >,
        >,
        /// Specifies whether advanced filters should be evaluated against an array of values instead of expecting a singular value. Defaults to `false`.
        pub advanced_filtering_on_arrays_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// An `azure_function_endpoint` block as defined below.
        pub azure_function_endpoint: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionAzureFunctionEndpoint,
            >,
        >,
        /// A `dead_letter_identity` block as defined below.
        ///
        /// > **Note:** `storage_blob_dead_letter_destination` must be specified when a `dead_letter_identity` is specified
        pub dead_letter_identity: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionDeadLetterIdentity,
            >,
        >,
        /// A `delivery_identity` block as defined below.
        pub delivery_identity: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionDeliveryIdentity,
            >,
        >,
        /// One or more `delivery_property` blocks as defined below.
        pub delivery_properties: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::eventgrid::SystemTopicEventSubscriptionDeliveryProperty,
                >,
            >,
        >,
        /// Specifies the event delivery schema for the event subscription. Possible values include: `EventGridSchema`, `CloudEventSchemaV1_0`, `CustomInputSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
        pub event_delivery_schema: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the id where the Event Hub is located.
        pub eventhub_endpoint_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the expiration time of the event subscription (Datetime Format `RFC 3339`).
        pub expiration_time_utc: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the id where the Hybrid Connection is located.
        pub hybrid_connection_endpoint_id: pulumi_gestalt_rust::Output<String>,
        /// A list of applicable event types that need to be part of the event subscription.
        pub included_event_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of labels to assign to the event subscription.
        pub labels: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The name which should be used for this Event Subscription. Changing this forces a new Event Subscription to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the System Topic exists. Changing this forces a new Event Subscription to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `retry_policy` block as defined below.
        pub retry_policy: pulumi_gestalt_rust::Output<
            super::super::types::eventgrid::SystemTopicEventSubscriptionRetryPolicy,
        >,
        /// Specifies the id where the Service Bus Queue is located.
        pub service_bus_queue_endpoint_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the id where the Service Bus Topic is located.
        pub service_bus_topic_endpoint_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `storage_blob_dead_letter_destination` block as defined below.
        pub storage_blob_dead_letter_destination: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionStorageBlobDeadLetterDestination,
            >,
        >,
        /// A `storage_queue_endpoint` block as defined below.
        pub storage_queue_endpoint: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionStorageQueueEndpoint,
            >,
        >,
        /// A `subject_filter` block as defined below.
        pub subject_filter: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::eventgrid::SystemTopicEventSubscriptionSubjectFilter,
            >,
        >,
        /// The System Topic where the Event Subscription should be created in. Changing this forces a new Event Subscription to be created.
        pub system_topic: pulumi_gestalt_rust::Output<String>,
        /// A `webhook_endpoint` block as defined below.
        ///
        /// > **NOTE:** One of `azure_function_endpoint`, `eventhub_endpoint_id`, `hybrid_connection_endpoint`, `hybrid_connection_endpoint_id`, `service_bus_queue_endpoint_id`, `service_bus_topic_endpoint_id`, `storage_queue_endpoint` or `webhook_endpoint` must be specified.
        pub webhook_endpoint: pulumi_gestalt_rust::Output<
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SystemTopicEventSubscriptionArgs,
    ) -> SystemTopicEventSubscriptionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let advanced_filter_binding_1 = args.advanced_filter.get_output(context);
        let advanced_filter_binding = advanced_filter_binding_1.get_inner();
        let advanced_filtering_on_arrays_enabled_binding_1 = args
            .advanced_filtering_on_arrays_enabled
            .get_output(context);
        let advanced_filtering_on_arrays_enabled_binding = advanced_filtering_on_arrays_enabled_binding_1
            .get_inner();
        let azure_function_endpoint_binding_1 = args
            .azure_function_endpoint
            .get_output(context);
        let azure_function_endpoint_binding = azure_function_endpoint_binding_1
            .get_inner();
        let dead_letter_identity_binding_1 = args
            .dead_letter_identity
            .get_output(context);
        let dead_letter_identity_binding = dead_letter_identity_binding_1.get_inner();
        let delivery_identity_binding_1 = args.delivery_identity.get_output(context);
        let delivery_identity_binding = delivery_identity_binding_1.get_inner();
        let delivery_properties_binding_1 = args.delivery_properties.get_output(context);
        let delivery_properties_binding = delivery_properties_binding_1.get_inner();
        let event_delivery_schema_binding_1 = args
            .event_delivery_schema
            .get_output(context);
        let event_delivery_schema_binding = event_delivery_schema_binding_1.get_inner();
        let eventhub_endpoint_id_binding_1 = args
            .eventhub_endpoint_id
            .get_output(context);
        let eventhub_endpoint_id_binding = eventhub_endpoint_id_binding_1.get_inner();
        let expiration_time_utc_binding_1 = args.expiration_time_utc.get_output(context);
        let expiration_time_utc_binding = expiration_time_utc_binding_1.get_inner();
        let hybrid_connection_endpoint_id_binding_1 = args
            .hybrid_connection_endpoint_id
            .get_output(context);
        let hybrid_connection_endpoint_id_binding = hybrid_connection_endpoint_id_binding_1
            .get_inner();
        let included_event_types_binding_1 = args
            .included_event_types
            .get_output(context);
        let included_event_types_binding = included_event_types_binding_1.get_inner();
        let labels_binding_1 = args.labels.get_output(context);
        let labels_binding = labels_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let retry_policy_binding_1 = args.retry_policy.get_output(context);
        let retry_policy_binding = retry_policy_binding_1.get_inner();
        let service_bus_queue_endpoint_id_binding_1 = args
            .service_bus_queue_endpoint_id
            .get_output(context);
        let service_bus_queue_endpoint_id_binding = service_bus_queue_endpoint_id_binding_1
            .get_inner();
        let service_bus_topic_endpoint_id_binding_1 = args
            .service_bus_topic_endpoint_id
            .get_output(context);
        let service_bus_topic_endpoint_id_binding = service_bus_topic_endpoint_id_binding_1
            .get_inner();
        let storage_blob_dead_letter_destination_binding_1 = args
            .storage_blob_dead_letter_destination
            .get_output(context);
        let storage_blob_dead_letter_destination_binding = storage_blob_dead_letter_destination_binding_1
            .get_inner();
        let storage_queue_endpoint_binding_1 = args
            .storage_queue_endpoint
            .get_output(context);
        let storage_queue_endpoint_binding = storage_queue_endpoint_binding_1
            .get_inner();
        let subject_filter_binding_1 = args.subject_filter.get_output(context);
        let subject_filter_binding = subject_filter_binding_1.get_inner();
        let system_topic_binding_1 = args.system_topic.get_output(context);
        let system_topic_binding = system_topic_binding_1.get_inner();
        let webhook_endpoint_binding_1 = args.webhook_endpoint.get_output(context);
        let webhook_endpoint_binding = webhook_endpoint_binding_1.get_inner();
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
            advanced_filter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("advancedFilter"),
            ),
            advanced_filtering_on_arrays_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("advancedFilteringOnArraysEnabled"),
            ),
            azure_function_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("azureFunctionEndpoint"),
            ),
            dead_letter_identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deadLetterIdentity"),
            ),
            delivery_identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deliveryIdentity"),
            ),
            delivery_properties: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deliveryProperties"),
            ),
            event_delivery_schema: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventDeliverySchema"),
            ),
            eventhub_endpoint_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventhubEndpointId"),
            ),
            expiration_time_utc: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expirationTimeUtc"),
            ),
            hybrid_connection_endpoint_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hybridConnectionEndpointId"),
            ),
            included_event_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("includedEventTypes"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            retry_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("retryPolicy"),
            ),
            service_bus_queue_endpoint_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceBusQueueEndpointId"),
            ),
            service_bus_topic_endpoint_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceBusTopicEndpointId"),
            ),
            storage_blob_dead_letter_destination: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageBlobDeadLetterDestination"),
            ),
            storage_queue_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageQueueEndpoint"),
            ),
            subject_filter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subjectFilter"),
            ),
            system_topic: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("systemTopic"),
            ),
            webhook_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("webhookEndpoint"),
            ),
        }
    }
}
