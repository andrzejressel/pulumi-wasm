#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_event_source {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEventSourceArgs {
        /// Specifying this limits the results to only those partner event sources with names that start with the specified prefix
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetEventSourceResult {
        /// ARN of the partner event source
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the SaaS partner that created the event source
        pub created_by: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Name of the event source
        pub name: pulumi_gestalt_rust::Output<String>,
        pub name_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// State of the event source (`ACTIVE` or `PENDING`)
        pub state: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetEventSourceArgs,
    ) -> GetEventSourceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_prefix_binding = args.name_prefix.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:cloudwatch/getEventSource:getEventSource".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetEventSourceResult {
            arn: o.get_field("arn"),
            created_by: o.get_field("createdBy"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            state: o.get_field("state"),
        }
    }
}
