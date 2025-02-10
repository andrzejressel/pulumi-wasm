#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_sink {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSinkArgs {
        /// ARN of the sink.
        #[builder(into)]
        pub sink_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Tags assigned to the sink.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSinkResult {
        /// ARN of the sink.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Name of the sink.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Random ID string that AWS generated as part of the sink ARN.
        pub sink_id: pulumi_gestalt_rust::Output<String>,
        pub sink_identifier: pulumi_gestalt_rust::Output<String>,
        /// Tags assigned to the sink.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSinkArgs,
    ) -> GetSinkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let sink_identifier_binding = args.sink_identifier.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:oam/getSink:getSink".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sinkIdentifier".into(),
                    value: sink_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSinkResult {
            arn: o.get_field("arn"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            sink_id: o.get_field("sinkId"),
            sink_identifier: o.get_field("sinkIdentifier"),
            tags: o.get_field("tags"),
        }
    }
}
