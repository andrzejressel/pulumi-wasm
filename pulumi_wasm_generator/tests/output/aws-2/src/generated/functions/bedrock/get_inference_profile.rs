pub mod get_inference_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInferenceProfileArgs {
        /// Inference Profile identifier.
        #[builder(into)]
        pub inference_profile_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetInferenceProfileResult {
        /// The time at which the inference profile was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// The description of the inference profile.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the inference profile.
        pub inference_profile_arn: pulumi_wasm_rust::Output<String>,
        pub inference_profile_id: pulumi_wasm_rust::Output<String>,
        /// The unique identifier of the inference profile.
        pub inference_profile_name: pulumi_wasm_rust::Output<String>,
        /// A list of information about each model in the inference profile. See `models`.
        pub models: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::bedrock::GetInferenceProfileModel>,
        >,
        /// The status of the inference profile. `ACTIVE` means that the inference profile is available to use.
        pub status: pulumi_wasm_rust::Output<String>,
        /// The type of the inference profile. `SYSTEM_DEFINED` means that the inference profile is defined by Amazon Bedrock. `APPLICATION` means that the inference profile is defined by the user.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// The time at which the inference profile was last updated.
        pub updated_at: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetInferenceProfileArgs) -> GetInferenceProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let inference_profile_id_binding = args.inference_profile_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:bedrock/getInferenceProfile:getInferenceProfile".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "inferenceProfileId".into(),
                    value: &inference_profile_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "inferenceProfileArn".into(),
                },
                register_interface::ResultField {
                    name: "inferenceProfileId".into(),
                },
                register_interface::ResultField {
                    name: "inferenceProfileName".into(),
                },
                register_interface::ResultField {
                    name: "models".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "updatedAt".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetInferenceProfileResult {
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            inference_profile_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inferenceProfileArn").unwrap(),
            ),
            inference_profile_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inferenceProfileId").unwrap(),
            ),
            inference_profile_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inferenceProfileName").unwrap(),
            ),
            models: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("models").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            updated_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updatedAt").unwrap(),
            ),
        }
    }
}
