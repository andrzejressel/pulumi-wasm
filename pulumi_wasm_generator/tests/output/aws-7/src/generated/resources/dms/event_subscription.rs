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
pub mod event_subscription {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventSubscriptionArgs {
        /// Whether the event subscription should be enabled.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of event categories to listen for, see `DescribeEventCategories` for a canonical list.
        #[builder(into)]
        pub event_categories: pulumi_wasm_rust::Output<Vec<String>>,
        /// Name of event subscription.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// SNS topic arn to send events on.
        #[builder(into)]
        pub sns_topic_arn: pulumi_wasm_rust::Output<String>,
        /// Ids of sources to listen to. If you don't specify a value, notifications are provided for all sources.
        #[builder(into, default)]
        pub source_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Type of source for events. Valid values: `replication-instance` or `replication-task`
        #[builder(into)]
        pub source_type: pulumi_wasm_rust::Output<String>,
        /// Map of resource tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EventSubscriptionResult {
        /// Amazon Resource Name (ARN) of the DMS Event Subscription.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Whether the event subscription should be enabled.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of event categories to listen for, see `DescribeEventCategories` for a canonical list.
        pub event_categories: pulumi_wasm_rust::Output<Vec<String>>,
        /// Name of event subscription.
        pub name: pulumi_wasm_rust::Output<String>,
        /// SNS topic arn to send events on.
        pub sns_topic_arn: pulumi_wasm_rust::Output<String>,
        /// Ids of sources to listen to. If you don't specify a value, notifications are provided for all sources.
        pub source_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Type of source for events. Valid values: `replication-instance` or `replication-task`
        pub source_type: pulumi_wasm_rust::Output<String>,
        /// Map of resource tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        let sns_topic_arn_binding = args.sns_topic_arn.get_inner();
        let source_ids_binding = args.source_ids.get_inner();
        let source_type_binding = args.source_type.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:dms/eventSubscription:EventSubscription".into(),
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
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "eventCategories".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            event_categories: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventCategories").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
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
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
