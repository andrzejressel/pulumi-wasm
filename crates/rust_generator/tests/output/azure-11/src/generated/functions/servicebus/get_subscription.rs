#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_subscription {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSubscriptionArgs {
        /// Specifies the name of the ServiceBus Subscription.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub namespace_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the ServiceBus Topic where the Service Bus Subscription exists.
        #[builder(into, default)]
        pub topic_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub topic_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSubscriptionResult {
        /// The idle interval after which the Subscription is automatically deleted.
        pub auto_delete_on_idle: pulumi_gestalt_rust::Output<String>,
        /// Does the ServiceBus Subscription have dead letter support on filter evaluation exceptions?
        pub dead_lettering_on_filter_evaluation_error: pulumi_gestalt_rust::Output<bool>,
        /// Does the Service Bus Subscription have dead letter support when a message expires?
        pub dead_lettering_on_message_expiration: pulumi_gestalt_rust::Output<bool>,
        /// The Default message timespan to live. This is the duration after which the message expires, starting from when the message is sent to Service Bus. This is the default value used when TimeToLive is not set on a message itself.
        pub default_message_ttl: pulumi_gestalt_rust::Output<String>,
        /// Are batched operations enabled on this ServiceBus Subscription?
        pub enable_batched_operations: pulumi_gestalt_rust::Output<bool>,
        /// The name of a Queue or Topic to automatically forward Dead Letter messages to.
        pub forward_dead_lettered_messages_to: pulumi_gestalt_rust::Output<String>,
        /// The name of a ServiceBus Queue or ServiceBus Topic where messages are automatically forwarded.
        pub forward_to: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The lock duration for the subscription.
        pub lock_duration: pulumi_gestalt_rust::Output<String>,
        /// The maximum number of deliveries.
        pub max_delivery_count: pulumi_gestalt_rust::Output<i32>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub namespace_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether or not this ServiceBus Subscription supports session.
        pub requires_session: pulumi_gestalt_rust::Output<bool>,
        pub resource_group_name: pulumi_gestalt_rust::Output<Option<String>>,
        pub topic_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub topic_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSubscriptionArgs,
    ) -> GetSubscriptionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let namespace_name_binding = args.namespace_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let topic_id_binding = args.topic_id.get_output(context);
        let topic_name_binding = args.topic_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:servicebus/getSubscription:getSubscription".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespaceName".into(),
                    value: namespace_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "topicId".into(),
                    value: topic_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "topicName".into(),
                    value: topic_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSubscriptionResult {
            auto_delete_on_idle: o.get_field("autoDeleteOnIdle"),
            dead_lettering_on_filter_evaluation_error: o
                .get_field("deadLetteringOnFilterEvaluationError"),
            dead_lettering_on_message_expiration: o
                .get_field("deadLetteringOnMessageExpiration"),
            default_message_ttl: o.get_field("defaultMessageTtl"),
            enable_batched_operations: o.get_field("enableBatchedOperations"),
            forward_dead_lettered_messages_to: o
                .get_field("forwardDeadLetteredMessagesTo"),
            forward_to: o.get_field("forwardTo"),
            id: o.get_field("id"),
            lock_duration: o.get_field("lockDuration"),
            max_delivery_count: o.get_field("maxDeliveryCount"),
            name: o.get_field("name"),
            namespace_name: o.get_field("namespaceName"),
            requires_session: o.get_field("requiresSession"),
            resource_group_name: o.get_field("resourceGroupName"),
            topic_id: o.get_field("topicId"),
            topic_name: o.get_field("topicName"),
        }
    }
}
