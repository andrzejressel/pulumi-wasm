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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod lite_subscription {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LiteSubscriptionArgs {
        /// The settings for this subscription's message delivery.
        /// Structure is documented below.
        #[builder(into, default)]
        pub delivery_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::pubsub::LiteSubscriptionDeliveryConfig>,
        >,
        /// Name of the subscription.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region of the pubsub lite topic.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A reference to a Topic resource.
        #[builder(into)]
        pub topic: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The zone of the pubsub lite topic.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LiteSubscriptionResult {
        /// The settings for this subscription's message delivery.
        /// Structure is documented below.
        pub delivery_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::pubsub::LiteSubscriptionDeliveryConfig>,
        >,
        /// Name of the subscription.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The region of the pubsub lite topic.
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        /// A reference to a Topic resource.
        pub topic: pulumi_gestalt_rust::Output<String>,
        /// The zone of the pubsub lite topic.
        pub zone: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LiteSubscriptionArgs,
    ) -> LiteSubscriptionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let delivery_config_binding = args.delivery_config.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let topic_binding = args.topic.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:pubsub/liteSubscription:LiteSubscription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deliveryConfig".into(),
                    value: delivery_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "topic".into(),
                    value: topic_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: zone_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LiteSubscriptionResult {
            delivery_config: o.get_field("deliveryConfig"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            topic: o.get_field("topic"),
            zone: o.get_field("zone"),
        }
    }
}
