pub mod get_model {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetModelArgs {
        /// Model identifier.
        #[builder(into)]
        pub model_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetModelResult {
        /// Customizations that the model supports.
        pub customizations_supporteds: pulumi_wasm_rust::Output<Vec<String>>,
        pub id: pulumi_wasm_rust::Output<String>,
        /// Inference types that the model supports.
        pub inference_types_supporteds: pulumi_wasm_rust::Output<Vec<String>>,
        /// Input modalities that the model supports.
        pub input_modalities: pulumi_wasm_rust::Output<Vec<String>>,
        /// Model ARN.
        pub model_arn: pulumi_wasm_rust::Output<String>,
        pub model_id: pulumi_wasm_rust::Output<String>,
        /// Model name.
        pub model_name: pulumi_wasm_rust::Output<String>,
        /// Output modalities that the model supports.
        pub output_modalities: pulumi_wasm_rust::Output<Vec<String>>,
        /// Model provider name.
        pub provider_name: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the model supports streaming.
        pub response_streaming_supported: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetModelArgs) -> GetModelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let model_id_binding = args.model_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:bedrockfoundation/getModel:getModel".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "modelId".into(),
                    value: &model_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "customizationsSupporteds".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "inferenceTypesSupporteds".into(),
                },
                register_interface::ResultField {
                    name: "inputModalities".into(),
                },
                register_interface::ResultField {
                    name: "modelArn".into(),
                },
                register_interface::ResultField {
                    name: "modelId".into(),
                },
                register_interface::ResultField {
                    name: "modelName".into(),
                },
                register_interface::ResultField {
                    name: "outputModalities".into(),
                },
                register_interface::ResultField {
                    name: "providerName".into(),
                },
                register_interface::ResultField {
                    name: "responseStreamingSupported".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetModelResult {
            customizations_supporteds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customizationsSupporteds").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            inference_types_supporteds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inferenceTypesSupporteds").unwrap(),
            ),
            input_modalities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inputModalities").unwrap(),
            ),
            model_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modelArn").unwrap(),
            ),
            model_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modelId").unwrap(),
            ),
            model_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modelName").unwrap(),
            ),
            output_modalities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outputModalities").unwrap(),
            ),
            provider_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("providerName").unwrap(),
            ),
            response_streaming_supported: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("responseStreamingSupported").unwrap(),
            ),
        }
    }
}
