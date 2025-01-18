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
pub mod event_subscription {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventSubscriptionArgs {
        /// A boolean flag to enable/disable the subscription. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of event categories for a SourceType that you want to subscribe to. See https://docs.aws.amazon.com/redshift/latest/mgmt/working-with-event-notifications.html or run `aws redshift describe-event-categories`.
        #[builder(into, default)]
        pub event_categories: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the Redshift event subscription.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The event severity to be published by the notification subscription. Valid options are `INFO` or `ERROR`. Default value of `INFO`.
        #[builder(into, default)]
        pub severity: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the SNS topic to send events to.
        #[builder(into)]
        pub sns_topic_arn: pulumi_wasm_rust::Output<String>,
        /// A list of identifiers of the event sources for which events will be returned. If not specified, then all sources are included in the response. If specified, a `source_type` must also be specified.
        #[builder(into, default)]
        pub source_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The type of source that will be generating the events. Valid options are `cluster`, `cluster-parameter-group`, `cluster-security-group`, `cluster-snapshot`, or `scheduled-action`. If not set, all sources will be subscribed to.
        #[builder(into, default)]
        pub source_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EventSubscriptionResult {
        /// Amazon Resource Name (ARN) of the Redshift event notification subscription
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The AWS customer account associated with the Redshift event notification subscription
        pub customer_aws_id: pulumi_wasm_rust::Output<String>,
        /// A boolean flag to enable/disable the subscription. Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of event categories for a SourceType that you want to subscribe to. See https://docs.aws.amazon.com/redshift/latest/mgmt/working-with-event-notifications.html or run `aws redshift describe-event-categories`.
        pub event_categories: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the Redshift event subscription.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The event severity to be published by the notification subscription. Valid options are `INFO` or `ERROR`. Default value of `INFO`.
        pub severity: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the SNS topic to send events to.
        pub sns_topic_arn: pulumi_wasm_rust::Output<String>,
        /// A list of identifiers of the event sources for which events will be returned. If not specified, then all sources are included in the response. If specified, a `source_type` must also be specified.
        pub source_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The type of source that will be generating the events. Valid options are `cluster`, `cluster-parameter-group`, `cluster-security-group`, `cluster-snapshot`, or `scheduled-action`. If not set, all sources will be subscribed to.
        pub source_type: pulumi_wasm_rust::Output<Option<String>>,
        pub status: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EventSubscriptionArgs) -> EventSubscriptionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let enabled_binding = args.enabled.get_inner();
        let event_categories_binding = args.event_categories.get_inner();
        let name_binding = args.name.get_inner();
        let severity_binding = args.severity.get_inner();
        let sns_topic_arn_binding = args.sns_topic_arn.get_inner();
        let source_ids_binding = args.source_ids.get_inner();
        let source_type_binding = args.source_type.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshift/eventSubscription:EventSubscription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "eventCategories".into(),
                    value: &event_categories_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "severity".into(),
                    value: &severity_binding,
                },
                register_interface::ObjectField {
                    name: "snsTopicArn".into(),
                    value: &sns_topic_arn_binding,
                },
                register_interface::ObjectField {
                    name: "sourceIds".into(),
                    value: &source_ids_binding,
                },
                register_interface::ObjectField {
                    name: "sourceType".into(),
                    value: &source_type_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "customerAwsId".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "eventCategories".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "severity".into(),
                },
                register_interface::ResultField {
                    name: "snsTopicArn".into(),
                },
                register_interface::ResultField {
                    name: "sourceIds".into(),
                },
                register_interface::ResultField {
                    name: "sourceType".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EventSubscriptionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            customer_aws_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerAwsId").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            event_categories: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventCategories").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            severity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("severity").unwrap(),
            ),
            sns_topic_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snsTopicArn").unwrap(),
            ),
            source_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceIds").unwrap(),
            ),
            source_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceType").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
