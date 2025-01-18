pub mod get_models {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetModelsArgs {
        /// Customization type to filter on. Valid values are `FINE_TUNING`.
        #[builder(into, default)]
        pub by_customization_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Inference type to filter on. Valid values are `ON_DEMAND` and `PROVISIONED`.
        #[builder(into, default)]
        pub by_inference_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Output modality to filter on. Valid values are `TEXT`, `IMAGE`, and `EMBEDDING`.
        #[builder(into, default)]
        pub by_output_modality: pulumi_wasm_rust::Output<Option<String>>,
        /// Model provider to filter on.
        #[builder(into, default)]
        pub by_provider: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetModelsResult {
        pub by_customization_type: pulumi_wasm_rust::Output<Option<String>>,
        pub by_inference_type: pulumi_wasm_rust::Output<Option<String>>,
        pub by_output_modality: pulumi_wasm_rust::Output<Option<String>>,
        pub by_provider: pulumi_wasm_rust::Output<Option<String>>,
        /// AWS region.
        pub id: pulumi_wasm_rust::Output<String>,
        /// List of model summary objects. See `model_summaries`.
        pub model_summaries: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::bedrockfoundation::GetModelsModelSummary>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetModelsArgs) -> GetModelsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let by_customization_type_binding = args.by_customization_type.get_inner();
        let by_inference_type_binding = args.by_inference_type.get_inner();
        let by_output_modality_binding = args.by_output_modality.get_inner();
        let by_provider_binding = args.by_provider.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:bedrockfoundation/getModels:getModels".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "byCustomizationType".into(),
                    value: &by_customization_type_binding,
                },
                register_interface::ObjectField {
                    name: "byInferenceType".into(),
                    value: &by_inference_type_binding,
                },
                register_interface::ObjectField {
                    name: "byOutputModality".into(),
                    value: &by_output_modality_binding,
                },
                register_interface::ObjectField {
                    name: "byProvider".into(),
                    value: &by_provider_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "byCustomizationType".into(),
                },
                register_interface::ResultField {
                    name: "byInferenceType".into(),
                },
                register_interface::ResultField {
                    name: "byOutputModality".into(),
                },
                register_interface::ResultField {
                    name: "byProvider".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "modelSummaries".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetModelsResult {
            by_customization_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("byCustomizationType").unwrap(),
            ),
            by_inference_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("byInferenceType").unwrap(),
            ),
            by_output_modality: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("byOutputModality").unwrap(),
            ),
            by_provider: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("byProvider").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            model_summaries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modelSummaries").unwrap(),
            ),
        }
    }
}
