#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_event_integration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEventIntegrationArgs {
        /// The AppIntegrations Event Integration name.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Metadata that you can assign to help organize the report plans you create.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetEventIntegrationResult {
        /// The ARN of the AppIntegrations Event Integration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The description of the Event Integration.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// A block that defines the configuration information for the event filter. The Event Filter block is documented below.
        pub event_filters: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::appintegrations::GetEventIntegrationEventFilter,
            >,
        >,
        /// The EventBridge bus.
        pub eventbridge_bus: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Metadata that you can assign to help organize the report plans you create.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetEventIntegrationArgs,
    ) -> GetEventIntegrationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:appintegrations/getEventIntegration:getEventIntegration".into(),
            version: super::super::super::get_version(),
            object: &[
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
        let o = context.invoke_resource(request);
        GetEventIntegrationResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            event_filters: o.get_field("eventFilters"),
            eventbridge_bus: o.get_field("eventbridgeBus"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
        }
    }
}
