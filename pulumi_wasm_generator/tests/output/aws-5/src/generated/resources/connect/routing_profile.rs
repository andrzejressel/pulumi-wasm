/// Provides an Amazon Connect Routing Profile resource. For more information see
/// [Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:connect:RoutingProfile
///     properties:
///       instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
///       name: example
///       defaultOutboundQueueId: 12345678-1234-1234-1234-123456789012
///       description: example description
///       mediaConcurrencies:
///         - channel: VOICE
///           concurrency: 1
///       queueConfigs:
///         - channel: VOICE
///           delay: 2
///           priority: 1
///           queueId: 12345678-1234-1234-1234-123456789012
///       tags:
///         Name: Example Routing Profile
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon Connect Routing Profiles using the `instance_id` and `routing_profile_id` separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:connect/routingProfile:RoutingProfile example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5
/// ```
pub mod routing_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RoutingProfileArgs {
        /// Specifies the default outbound queue for the Routing Profile.
        #[builder(into)]
        pub default_outbound_queue_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the description of the Routing Profile.
        #[builder(into)]
        pub description: pulumi_wasm_rust::Output<String>,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// One or more `media_concurrencies` blocks that specify the channels that agents can handle in the Contact Control Panel (CCP) for this Routing Profile. The `media_concurrencies` block is documented below.
        #[builder(into)]
        pub media_concurrencies: pulumi_wasm_rust::Output<
            Vec<super::super::types::connect::RoutingProfileMediaConcurrency>,
        >,
        /// Specifies the name of the Routing Profile.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `queue_configs` blocks that specify the inbound queues associated with the routing profile. If no queue is added, the agent only can make outbound calls. The `queue_configs` block is documented below.
        #[builder(into, default)]
        pub queue_configs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::connect::RoutingProfileQueueConfig>>,
        >,
        /// Tags to apply to the Routing Profile. If configured with a provider
        /// `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RoutingProfileResult {
        /// The Amazon Resource Name (ARN) of the Routing Profile.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Specifies the default outbound queue for the Routing Profile.
        pub default_outbound_queue_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the description of the Routing Profile.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// One or more `media_concurrencies` blocks that specify the channels that agents can handle in the Contact Control Panel (CCP) for this Routing Profile. The `media_concurrencies` block is documented below.
        pub media_concurrencies: pulumi_wasm_rust::Output<
            Vec<super::super::types::connect::RoutingProfileMediaConcurrency>,
        >,
        /// Specifies the name of the Routing Profile.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `queue_configs` blocks that specify the inbound queues associated with the routing profile. If no queue is added, the agent only can make outbound calls. The `queue_configs` block is documented below.
        pub queue_configs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::connect::RoutingProfileQueueConfig>>,
        >,
        /// The identifier for the Routing Profile.
        pub routing_profile_id: pulumi_wasm_rust::Output<String>,
        /// Tags to apply to the Routing Profile. If configured with a provider
        /// `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(name: &str, args: RoutingProfileArgs) -> RoutingProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let default_outbound_queue_id_binding = args
            .default_outbound_queue_id
            .get_inner();
        let description_binding = args.description.get_inner();
        let instance_id_binding = args.instance_id.get_inner();
        let media_concurrencies_binding = args.media_concurrencies.get_inner();
        let name_binding = args.name.get_inner();
        let queue_configs_binding = args.queue_configs.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:connect/routingProfile:RoutingProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultOutboundQueueId".into(),
                    value: &default_outbound_queue_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "mediaConcurrencies".into(),
                    value: &media_concurrencies_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "queueConfigs".into(),
                    value: &queue_configs_binding,
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
                    name: "defaultOutboundQueueId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "mediaConcurrencies".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "queueConfigs".into(),
                },
                register_interface::ResultField {
                    name: "routingProfileId".into(),
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
        RoutingProfileResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            default_outbound_queue_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultOutboundQueueId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            media_concurrencies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mediaConcurrencies").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            queue_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queueConfigs").unwrap(),
            ),
            routing_profile_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routingProfileId").unwrap(),
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
