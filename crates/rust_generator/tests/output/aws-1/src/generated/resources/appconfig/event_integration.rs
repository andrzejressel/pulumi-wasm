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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod event_integration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventIntegrationArgs {
        /// Description of the Event Integration.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Block that defines the configuration information for the event filter. The Event Filter block is documented below.
        #[builder(into)]
        pub event_filter: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appconfig::EventIntegrationEventFilter,
        >,
        /// EventBridge bus.
        #[builder(into)]
        pub eventbridge_bus: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the Event Integration.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Tags to apply to the Event Integration. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EventIntegrationResult {
        /// ARN of the Event Integration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the Event Integration.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Block that defines the configuration information for the event filter. The Event Filter block is documented below.
        pub event_filter: pulumi_gestalt_rust::Output<
            super::super::types::appconfig::EventIntegrationEventFilter,
        >,
        /// EventBridge bus.
        pub eventbridge_bus: pulumi_gestalt_rust::Output<String>,
        /// Name of the Event Integration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Tags to apply to the Event Integration. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: EventIntegrationArgs,
    ) -> EventIntegrationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let event_filter_binding = args.event_filter.get_output(context);
        let eventbridge_bus_binding = args.eventbridge_bus.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appconfig/eventIntegration:EventIntegration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventFilter".into(),
                    value: event_filter_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventbridgeBus".into(),
                    value: eventbridge_bus_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EventIntegrationResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            event_filter: o.get_field("eventFilter"),
            eventbridge_bus: o.get_field("eventbridgeBus"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
