#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_image_pipelines {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImagePipelinesArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::imagebuilder::GetImagePipelinesFilter>,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GetImagePipelinesResult {
        /// Set of ARNs of the matched Image Builder Image Pipelines.
        pub arns: pulumi_gestalt_rust::Output<Vec<String>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::super::types::imagebuilder::GetImagePipelinesFilter>,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Set of names of the matched Image Builder Image Pipelines.
        pub names: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetImagePipelinesArgs,
    ) -> GetImagePipelinesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:imagebuilder/getImagePipelines:getImagePipelines".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetImagePipelinesResult {
            arns: o.get_field("arns"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            names: o.get_field("names"),
        }
    }
}
