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
    pub fn invoke() -> GetInferenceProfilesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:bedrock/getInferenceProfiles:getInferenceProfiles".into(),
            object: Vec::from([]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "inferenceProfileSummaries".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetInferenceProfilesResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            inference_profile_summaries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inferenceProfileSummaries").unwrap(),
            ),
        }
    }
}