#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_custom_models {
    #[allow(dead_code)]
    pub struct GetCustomModelsResult {
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Model summaries.
        pub model_summaries: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::bedrock::GetCustomModelsModelSummary>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_gestalt_rust::Context) -> GetCustomModelsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:bedrock/getCustomModels:getCustomModels".into(),
            version: super::super::super::get_version(),
            object: &[],
        };
        let o = context.invoke_resource(request);
        GetCustomModelsResult {
            id: o.get_field("id"),
            model_summaries: o.get_field("modelSummaries"),
        }
    }
}
