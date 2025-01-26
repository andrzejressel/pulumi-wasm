pub mod get_inference_profiles {
    #[allow(dead_code)]
    pub struct GetInferenceProfilesResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// List of inference profile summary objects. See `inference_profile_summaries`.
        pub inference_profile_summaries: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::bedrock::GetInferenceProfilesInferenceProfileSummary,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
    ) -> GetInferenceProfilesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:bedrock/getInferenceProfiles:getInferenceProfiles".into(),
            version: super::super::super::get_version(),
            object: Vec::from([]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetInferenceProfilesResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            inference_profile_summaries: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("inferenceProfileSummaries"),
            ),
        }
    }
}
