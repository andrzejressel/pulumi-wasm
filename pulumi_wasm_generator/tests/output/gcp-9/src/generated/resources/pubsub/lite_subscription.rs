/// A named resource representing the stream of messages from a single,
/// specific topic, to be delivered to the subscribing application.
///
///
/// To get more information about Subscription, see:
///
/// * [API documentation](https://cloud.google.com/pubsub/lite/docs/reference/rest/v1/admin.projects.locations.subscriptions)
/// * How-to Guides
///     * [Managing Subscriptions](https://cloud.google.com/pubsub/lite/docs/subscriptions)
///
/// ## Example Usage
///
/// ### Pubsub Lite Subscription Basic
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:pubsub:LiteTopic
///     properties:
///       name: example-topic
///       project: ${project.number}
///       partitionConfig:
///         count: 1
///         capacity:
///           publishMibPerSec: 4
///           subscribeMibPerSec: 8
///       retentionConfig:
///         perPartitionBytes: 3.221225472e+10
///   exampleLiteSubscription:
///     type: gcp:pubsub:LiteSubscription
///     name: example
///     properties:
///       name: example-subscription
///       topic: ${example.name}
///       deliveryConfig:
///         deliveryRequirement: DELIVER_AFTER_STORED
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Subscription can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{zone}}/subscriptions/{{name}}`
///
/// * `{{project}}/{{zone}}/{{name}}`
///
/// * `{{zone}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Subscription can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:pubsub/liteSubscription:LiteSubscription default projects/{{project}}/locations/{{zone}}/subscriptions/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:pubsub/liteSubscription:LiteSubscription default {{project}}/{{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:pubsub/liteSubscription:LiteSubscription default {{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:pubsub/liteSubscription:LiteSubscription default {{name}}
/// ```
///
pub mod lite_subscription {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LiteSubscriptionArgs {
        /// The settings for this subscription's message delivery.
        /// Structure is documented below.
        #[builder(into, default)]
        pub delivery_config: pulumi_wasm_rust::Output<
            Option<super::super::types::pubsub::LiteSubscriptionDeliveryConfig>,
        >,
        /// Name of the subscription.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The region of the pubsub lite topic.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// A reference to a Topic resource.
        #[builder(into)]
        pub topic: pulumi_wasm_rust::Output<String>,
        /// The zone of the pubsub lite topic.
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LiteSubscriptionResult {
        /// The settings for this subscription's message delivery.
        /// Structure is documented below.
        pub delivery_config: pulumi_wasm_rust::Output<
            Option<super::super::types::pubsub::LiteSubscriptionDeliveryConfig>,
        >,
        /// Name of the subscription.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The region of the pubsub lite topic.
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// A reference to a Topic resource.
        pub topic: pulumi_wasm_rust::Output<String>,
        /// The zone of the pubsub lite topic.
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LiteSubscriptionArgs) -> LiteSubscriptionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let delivery_config_binding = args.delivery_config.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let topic_binding = args.topic.get_inner();
        let zone_binding = args.zone.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:pubsub/liteSubscription:LiteSubscription".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deliveryConfig".into(),
                    value: &delivery_config_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "topic".into(),
                    value: &topic_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "deliveryConfig".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "topic".into(),
                },
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LiteSubscriptionResult {
            delivery_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deliveryConfig").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            topic: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("topic").unwrap(),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}
