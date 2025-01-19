/// Provides an Amazon AppIntegrations Event Integration resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:appconfig:EventIntegration
///     properties:
///       name: example-name
///       description: Example Description
///       eventbridgeBus: default
///       eventFilter:
///         source: aws.partner/examplepartner.com
///       tags:
///         Name: Example Event Integration
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon AppIntegrations Event Integrations using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:appconfig/eventIntegration:EventIntegration example example-name
/// ```
pub mod event_integration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventIntegrationArgs {
        /// Description of the Event Integration.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Block that defines the configuration information for the event filter. The Event Filter block is documented below.
        #[builder(into)]
        pub event_filter: pulumi_wasm_rust::Output<
            super::super::types::appconfig::EventIntegrationEventFilter,
        >,
        /// EventBridge bus.
        #[builder(into)]
        pub eventbridge_bus: pulumi_wasm_rust::Output<String>,
        /// Name of the Event Integration.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Tags to apply to the Event Integration. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EventIntegrationResult {
        /// ARN of the Event Integration.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Description of the Event Integration.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Block that defines the configuration information for the event filter. The Event Filter block is documented below.
        pub event_filter: pulumi_wasm_rust::Output<
            super::super::types::appconfig::EventIntegrationEventFilter,
        >,
        /// EventBridge bus.
        pub eventbridge_bus: pulumi_wasm_rust::Output<String>,
        /// Name of the Event Integration.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Tags to apply to the Event Integration. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EventIntegrationArgs) -> EventIntegrationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let event_filter_binding = args.event_filter.get_inner();
        let eventbridge_bus_binding = args.eventbridge_bus.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appconfig/eventIntegration:EventIntegration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "eventFilter".into(),
                    value: &event_filter_binding,
                },
                register_interface::ObjectField {
                    name: "eventbridgeBus".into(),
                    value: &eventbridge_bus_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "eventFilter".into(),
                },
                register_interface::ResultField {
                    name: "eventbridgeBus".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
        EventIntegrationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            event_filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventFilter").unwrap(),
            ),
            eventbridge_bus: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventbridgeBus").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
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
