#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_pipeline {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPipelineArgs {
        /// ID of the pipeline.
        #[builder(into)]
        pub pipeline_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetPipelineResult {
        /// Description of Pipeline.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Name of Pipeline.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub pipeline_id: pulumi_gestalt_rust::Output<String>,
        /// Map of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPipelineArgs,
    ) -> GetPipelineResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let pipeline_id_binding = args.pipeline_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:datapipeline/getPipeline:getPipeline".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pipelineId".into(),
                    value: pipeline_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPipelineResult {
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            pipeline_id: o.get_field("pipelineId"),
            tags: o.get_field("tags"),
        }
    }
}
