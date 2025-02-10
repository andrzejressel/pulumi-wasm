#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_inference_profiles {
    #[allow(dead_code)]
    pub struct GetInferenceProfilesResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// List of inference profile summary objects. See `inference_profile_summaries`.
        pub inference_profile_summaries: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::bedrock::GetInferenceProfilesInferenceProfileSummary,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_gestalt_rust::Context) -> GetInferenceProfilesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:bedrock/getInferenceProfiles:getInferenceProfiles".into(),
            version: super::super::super::get_version(),
            object: &[],
        };
        let o = context.invoke_resource(request);
        GetInferenceProfilesResult {
            id: o.get_field("id"),
            inference_profile_summaries: o.get_field("inferenceProfileSummaries"),
        }
    }
}
