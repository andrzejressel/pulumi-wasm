pub mod get_custom_model {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCustomModelArgs {
        /// Name or ARN of the custom model.
        #[builder(into)]
        pub model_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetCustomModelResult {
        /// ARN of the base model.
        pub base_model_arn: pulumi_wasm_rust::Output<String>,
        /// Creation time of the model.
        pub creation_time: pulumi_wasm_rust::Output<String>,
        /// Hyperparameter values associated with this model.
        pub hyperparameters: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub id: pulumi_wasm_rust::Output<String>,
        /// Job ARN associated with this model.
        pub job_arn: pulumi_wasm_rust::Output<String>,
        /// Job name associated with this model.
        pub job_name: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of tags for the fine-tuning job.
        pub job_tags: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// ARN associated with this model.
        pub model_arn: pulumi_wasm_rust::Output<String>,
        pub model_id: pulumi_wasm_rust::Output<String>,
        /// The custom model is encrypted at rest using this key.
        pub model_kms_key_arn: pulumi_wasm_rust::Output<String>,
        /// Model name associated with this model.
        pub model_name: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of tags for the model.
        pub model_tags: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output data configuration associated with this custom model.
        pub output_data_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::bedrock::GetCustomModelOutputDataConfig>,
        >,
        /// Information about the training dataset.
        pub training_data_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::bedrock::GetCustomModelTrainingDataConfig>,
        >,
        /// Metrics associated with the customization job.
        pub training_metrics: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::bedrock::GetCustomModelTrainingMetric>,
        >,
        /// Information about the validation dataset.
        pub validation_data_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::bedrock::GetCustomModelValidationDataConfig>,
        >,
        /// The loss metric for each validator that you provided.
        pub validation_metrics: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::bedrock::GetCustomModelValidationMetric>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetCustomModelArgs) -> GetCustomModelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let model_id_binding = args.model_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:bedrock/getCustomModel:getCustomModel".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "modelId".into(),
                    value: &model_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "baseModelArn".into(),
                },
                register_interface::ResultField {
                    name: "creationTime".into(),
                },
                register_interface::ResultField {
                    name: "hyperparameters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "jobArn".into(),
                },
                register_interface::ResultField {
                    name: "jobName".into(),
                },
                register_interface::ResultField {
                    name: "jobTags".into(),
                },
                register_interface::ResultField {
                    name: "modelArn".into(),
                },
                register_interface::ResultField {
                    name: "modelId".into(),
                },
                register_interface::ResultField {
                    name: "modelKmsKeyArn".into(),
                },
                register_interface::ResultField {
                    name: "modelName".into(),
                },
                register_interface::ResultField {
                    name: "modelTags".into(),
                },
                register_interface::ResultField {
                    name: "outputDataConfigs".into(),
                },
                register_interface::ResultField {
                    name: "trainingDataConfigs".into(),
                },
                register_interface::ResultField {
                    name: "trainingMetrics".into(),
                },
                register_interface::ResultField {
                    name: "validationDataConfigs".into(),
                },
                register_interface::ResultField {
                    name: "validationMetrics".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCustomModelResult {
            base_model_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("baseModelArn").unwrap(),
            ),
            creation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTime").unwrap(),
            ),
            hyperparameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hyperparameters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            job_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jobArn").unwrap(),
            ),
            job_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jobName").unwrap(),
            ),
            job_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jobTags").unwrap(),
            ),
            model_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modelArn").unwrap(),
            ),
            model_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modelId").unwrap(),
            ),
            model_kms_key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modelKmsKeyArn").unwrap(),
            ),
            model_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modelName").unwrap(),
            ),
            model_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modelTags").unwrap(),
            ),
            output_data_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outputDataConfigs").unwrap(),
            ),
            training_data_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trainingDataConfigs").unwrap(),
            ),
            training_metrics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trainingMetrics").unwrap(),
            ),
            validation_data_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validationDataConfigs").unwrap(),
            ),
            validation_metrics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validationMetrics").unwrap(),
            ),
        }
    }
}
