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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventIntegrationArgs {
        /// Description of the Event Integration.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Block that defines the configuration information for the event filter. The Event Filter block is documented below.
        #[builder(into)]
        pub event_filter: pulumi_wasm_rust::InputOrOutput<
            super::super::types::appconfig::EventIntegrationEventFilter,
        >,
        /// EventBridge bus.
        #[builder(into)]
        pub eventbridge_bus: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the Event Integration.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Tags to apply to the Event Integration. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: EventIntegrationArgs,
    ) -> EventIntegrationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let event_filter_binding = args.event_filter.get_output(context).get_inner();
        let eventbridge_bus_binding = args
            .eventbridge_bus
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        EventIntegrationResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            event_filter: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("eventFilter"),
            ),
            eventbridge_bus: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("eventbridgeBus"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
