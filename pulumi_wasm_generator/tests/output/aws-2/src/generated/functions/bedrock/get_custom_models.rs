pub mod get_custom_models {
    #[allow(dead_code)]
    pub struct GetCustomModelsResult {
        pub id: pulumi_wasm_rust::Output<String>,
        /// Model summaries.
        pub model_summaries: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::bedrock::GetCustomModelsModelSummary>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_wasm_rust::PulumiContext) -> GetCustomModelsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:bedrock/getCustomModels:getCustomModels".into(),
            version: super::super::super::get_version(),
            object: Vec::from([]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "modelSummaries".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCustomModelsResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            model_summaries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modelSummaries").unwrap(),
            ),
        }
    }
}
