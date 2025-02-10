/// Provides a DMS (Data Migration Service) event subscription resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:dms:EventSubscription
///     properties:
///       enabled: true
///       eventCategories:
///         - creation
///         - failure
///       name: my-favorite-event-subscription
///       snsTopicArn: ${exampleAwsSnsTopic.arn}
///       sourceIds:
///         - ${exampleAwsDmsReplicationTask.replicationTaskId}
///       sourceType: replication-task
///       tags:
///         Name: example
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import event subscriptions using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:dms/eventSubscription:EventSubscription test my-awesome-event-subscription
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod event_subscription {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventSubscriptionArgs {
        /// Whether the event subscription should be enabled.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// List of event categories to listen for, see `DescribeEventCategories` for a canonical list.
        #[builder(into)]
        pub event_categories: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Name of event subscription.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// SNS topic arn to send events on.
        #[builder(into)]
        pub sns_topic_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Ids of sources to listen to. If you don't specify a value, notifications are provided for all sources.
        #[builder(into, default)]
        pub source_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Type of source for events. Valid values: `replication-instance` or `replication-task`
        #[builder(into)]
        pub source_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of resource tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EventSubscriptionResult {
        /// Amazon Resource Name (ARN) of the DMS Event Subscription.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Whether the event subscription should be enabled.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// List of event categories to listen for, see `DescribeEventCategories` for a canonical list.
        pub event_categories: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Name of event subscription.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// SNS topic arn to send events on.
        pub sns_topic_arn: pulumi_gestalt_rust::Output<String>,
        /// Ids of sources to listen to. If you don't specify a value, notifications are provided for all sources.
        pub source_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Type of source for events. Valid values: `replication-instance` or `replication-task`
        pub source_type: pulumi_gestalt_rust::Output<String>,
        /// Map of resource tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EventSubscriptionArgs,
    ) -> EventSubscriptionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let enabled_binding = args.enabled.get_output(context);
        let event_categories_binding = args.event_categories.get_output(context);
        let name_binding = args.name.get_output(context);
        let sns_topic_arn_binding = args.sns_topic_arn.get_output(context);
        let source_ids_binding = args.source_ids.get_output(context);
        let source_type_binding = args.source_type.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:dms/eventSubscription:EventSubscription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventCategories".into(),
                    value: event_categories_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snsTopicArn".into(),
                    value: sns_topic_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceIds".into(),
                    value: source_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceType".into(),
                    value: source_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EventSubscriptionResult {
            arn: o.get_field("arn"),
            enabled: o.get_field("enabled"),
            event_categories: o.get_field("eventCategories"),
            name: o.get_field("name"),
            sns_topic_arn: o.get_field("snsTopicArn"),
            source_ids: o.get_field("sourceIds"),
            source_type: o.get_field("sourceType"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
