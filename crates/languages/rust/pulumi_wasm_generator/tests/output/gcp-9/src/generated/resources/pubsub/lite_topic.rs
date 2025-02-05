/// A named resource to which messages are sent by publishers.
///
///
/// To get more information about Topic, see:
///
/// * [API documentation](https://cloud.google.com/pubsub/lite/docs/reference/rest/v1/admin.projects.locations.topics)
/// * How-to Guides
///     * [Managing Topics](https://cloud.google.com/pubsub/lite/docs/topics)
///
/// ## Example Usage
///
/// ### Pubsub Lite Topic Basic
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:pubsub:LiteReservation
///     properties:
///       name: example-reservation
///       project: ${project.number}
///       throughputCapacity: 2
///   exampleLiteTopic:
///     type: gcp:pubsub:LiteTopic
///     name: example
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
///       reservationConfig:
///         throughputReservation: ${example.name}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Topic can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{zone}}/topics/{{name}}`
///
/// * `{{project}}/{{zone}}/{{name}}`
///
/// * `{{zone}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Topic can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:pubsub/liteTopic:LiteTopic default projects/{{project}}/locations/{{zone}}/topics/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:pubsub/liteTopic:LiteTopic default {{project}}/{{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:pubsub/liteTopic:LiteTopic default {{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:pubsub/liteTopic:LiteTopic default {{name}}
/// ```
///
pub mod lite_topic {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LiteTopicArgs {
        /// Name of the topic.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The settings for this topic's partitions.
        /// Structure is documented below.
        #[builder(into, default)]
        pub partition_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::pubsub::LiteTopicPartitionConfig>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The region of the pubsub lite topic.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The settings for this topic's Reservation usage.
        /// Structure is documented below.
        #[builder(into, default)]
        pub reservation_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::pubsub::LiteTopicReservationConfig>,
        >,
        /// The settings for a topic's message retention.
        /// Structure is documented below.
        #[builder(into, default)]
        pub retention_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::pubsub::LiteTopicRetentionConfig>,
        >,
        /// The zone of the pubsub lite topic.
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LiteTopicResult {
        /// Name of the topic.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The settings for this topic's partitions.
        /// Structure is documented below.
        pub partition_config: pulumi_wasm_rust::Output<
            Option<super::super::types::pubsub::LiteTopicPartitionConfig>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The region of the pubsub lite topic.
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// The settings for this topic's Reservation usage.
        /// Structure is documented below.
        pub reservation_config: pulumi_wasm_rust::Output<
            Option<super::super::types::pubsub::LiteTopicReservationConfig>,
        >,
        /// The settings for a topic's message retention.
        /// Structure is documented below.
        pub retention_config: pulumi_wasm_rust::Output<
            Option<super::super::types::pubsub::LiteTopicRetentionConfig>,
        >,
        /// The zone of the pubsub lite topic.
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LiteTopicArgs,
    ) -> LiteTopicResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let partition_config_binding = args
            .partition_config
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let reservation_config_binding = args
            .reservation_config
            .get_output(context)
            .get_inner();
        let retention_config_binding = args
            .retention_config
            .get_output(context)
            .get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:pubsub/liteTopic:LiteTopic".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "partitionConfig".into(),
                    value: &partition_config_binding,
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
                    name: "reservationConfig".into(),
                    value: &reservation_config_binding,
                },
                register_interface::ObjectField {
                    name: "retentionConfig".into(),
                    value: &retention_config_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LiteTopicResult {
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            partition_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("partitionConfig"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            reservation_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("reservationConfig"),
            ),
            retention_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("retentionConfig"),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
