#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_pipeline_definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPipelineDefinitionArgs {
        /// Parameter values used in the pipeline definition. See below
        #[builder(into, default)]
        pub parameter_values: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::datapipeline::GetPipelineDefinitionParameterValue,
                >,
            >,
        >,
        /// ID of the pipeline.
        #[builder(into)]
        pub pipeline_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPipelineDefinitionResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Parameter objects used in the pipeline definition. See below
        pub parameter_objects: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::datapipeline::GetPipelineDefinitionParameterObject,
            >,
        >,
        /// Parameter values used in the pipeline definition. See below
        pub parameter_values: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::datapipeline::GetPipelineDefinitionParameterValue,
                >,
            >,
        >,
        pub pipeline_id: pulumi_gestalt_rust::Output<String>,
        /// Objects defined in the pipeline. See below
        pub pipeline_objects: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::datapipeline::GetPipelineDefinitionPipelineObject,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPipelineDefinitionArgs,
    ) -> GetPipelineDefinitionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let parameter_values_binding = args.parameter_values.get_output(context);
        let pipeline_id_binding = args.pipeline_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:datapipeline/getPipelineDefinition:getPipelineDefinition".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameterValues".into(),
                    value: parameter_values_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pipelineId".into(),
                    value: pipeline_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPipelineDefinitionResult {
            id: o.get_field("id"),
            parameter_objects: o.get_field("parameterObjects"),
            parameter_values: o.get_field("parameterValues"),
            pipeline_id: o.get_field("pipelineId"),
            pipeline_objects: o.get_field("pipelineObjects"),
        }
    }
}
