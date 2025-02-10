/// Provides a Redshift event subscription resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   default:
///     type: aws:redshift:Cluster
///     properties:
///       clusterIdentifier: default
///       databaseName: default
///   defaultTopic:
///     type: aws:sns:Topic
///     name: default
///     properties:
///       name: redshift-events
///   defaultEventSubscription:
///     type: aws:redshift:EventSubscription
///     name: default
///     properties:
///       name: redshift-event-sub
///       snsTopicArn: ${defaultTopic.arn}
///       sourceType: cluster
///       sourceIds:
///         - ${default.id}
///       severity: INFO
///       eventCategories:
///         - configuration
///         - management
///         - monitoring
///         - security
///       tags:
///         Name: default
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Event Subscriptions using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/eventSubscription:EventSubscription default redshift-event-sub
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod event_subscription {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventSubscriptionArgs {
        /// A boolean flag to enable/disable the subscription. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A list of event categories for a SourceType that you want to subscribe to. See https://docs.aws.amazon.com/redshift/latest/mgmt/working-with-event-notifications.html or run `aws redshift describe-event-categories`.
        #[builder(into, default)]
        pub event_categories: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of the Redshift event subscription.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The event severity to be published by the notification subscription. Valid options are `INFO` or `ERROR`. Default value of `INFO`.
        #[builder(into, default)]
        pub severity: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the SNS topic to send events to.
        #[builder(into)]
        pub sns_topic_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of identifiers of the event sources for which events will be returned. If not specified, then all sources are included in the response. If specified, a `source_type` must also be specified.
        #[builder(into, default)]
        pub source_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The type of source that will be generating the events. Valid options are `cluster`, `cluster-parameter-group`, `cluster-security-group`, `cluster-snapshot`, or `scheduled-action`. If not set, all sources will be subscribed to.
        #[builder(into, default)]
        pub source_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EventSubscriptionResult {
        /// Amazon Resource Name (ARN) of the Redshift event notification subscription
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The AWS customer account associated with the Redshift event notification subscription
        pub customer_aws_id: pulumi_gestalt_rust::Output<String>,
        /// A boolean flag to enable/disable the subscription. Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A list of event categories for a SourceType that you want to subscribe to. See https://docs.aws.amazon.com/redshift/latest/mgmt/working-with-event-notifications.html or run `aws redshift describe-event-categories`.
        pub event_categories: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The name of the Redshift event subscription.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The event severity to be published by the notification subscription. Valid options are `INFO` or `ERROR`. Default value of `INFO`.
        pub severity: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ARN of the SNS topic to send events to.
        pub sns_topic_arn: pulumi_gestalt_rust::Output<String>,
        /// A list of identifiers of the event sources for which events will be returned. If not specified, then all sources are included in the response. If specified, a `source_type` must also be specified.
        pub source_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The type of source that will be generating the events. Valid options are `cluster`, `cluster-parameter-group`, `cluster-security-group`, `cluster-snapshot`, or `scheduled-action`. If not set, all sources will be subscribed to.
        pub source_type: pulumi_gestalt_rust::Output<Option<String>>,
        pub status: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        let severity_binding = args.severity.get_output(context);
        let sns_topic_arn_binding = args.sns_topic_arn.get_output(context);
        let source_ids_binding = args.source_ids.get_output(context);
        let source_type_binding = args.source_type.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:redshift/eventSubscription:EventSubscription".into(),
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
                    name: "severity".into(),
                    value: severity_binding.get_id(),
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
            customer_aws_id: o.get_field("customerAwsId"),
            enabled: o.get_field("enabled"),
            event_categories: o.get_field("eventCategories"),
            name: o.get_field("name"),
            severity: o.get_field("severity"),
            sns_topic_arn: o.get_field("snsTopicArn"),
            source_ids: o.get_field("sourceIds"),
            source_type: o.get_field("sourceType"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
