/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleCustomModel:
///     type: aws:bedrock:CustomModel
///     name: example
///     properties:
///       customModelName: example-model
///       jobName: example-job-1
///       baseModelIdentifier: ${example.modelArn}
///       roleArn: ${exampleAwsIamRole.arn}
///       hyperparameters:
///         epochCount: '1'
///         batchSize: '1'
///         learningRate: '0.005'
///         learningRateWarmupSteps: '0'
///       outputDataConfig:
///         s3Uri: s3://${output.id}/data/
///       trainingDataConfig:
///         s3Uri: s3://${training.id}/data/train.jsonl
/// variables:
///   example:
///     fn::invoke:
///       Function: aws:bedrockfoundation:getModel
///       Arguments:
///         modelId: amazon.titan-text-express-v1
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Bedrock custom model using the `job_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:bedrock/customModel:CustomModel example arn:aws:bedrock:us-west-2:123456789012:model-customization-job/amazon.titan-text-express-v1:0:8k/1y5n57gh5y2e
/// ```
pub mod custom_model {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomModelArgs {
        /// The Amazon Resource Name (ARN) of the base model.
        #[builder(into)]
        pub base_model_identifier: pulumi_wasm_rust::Output<String>,
        /// The custom model is encrypted at rest using this key. Specify the key ARN.
        #[builder(into, default)]
        pub custom_model_kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Name for the custom model.
        #[builder(into)]
        pub custom_model_name: pulumi_wasm_rust::Output<String>,
        /// The customization type. Valid values: `FINE_TUNING`, `CONTINUED_PRE_TRAINING`.
        #[builder(into, default)]
        pub customization_type: pulumi_wasm_rust::Output<Option<String>>,
        /// [Parameters](https://docs.aws.amazon.com/bedrock/latest/userguide/custom-models-hp.html) related to tuning the model.
        #[builder(into)]
        pub hyperparameters: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A name for the customization job.
        #[builder(into)]
        pub job_name: pulumi_wasm_rust::Output<String>,
        /// S3 location for the output data.
        #[builder(into, default)]
        pub output_data_config: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::CustomModelOutputDataConfig>,
        >,
        /// The Amazon Resource Name (ARN) of an IAM role that Bedrock can assume to perform tasks on your behalf.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the customization job and custom model. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::CustomModelTimeouts>,
        >,
        /// Information about the training dataset.
        #[builder(into, default)]
        pub training_data_config: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::CustomModelTrainingDataConfig>,
        >,
        /// Information about the validation dataset.
        #[builder(into, default)]
        pub validation_data_config: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::CustomModelValidationDataConfig>,
        >,
        /// Configuration parameters for the private Virtual Private Cloud (VPC) that contains the resources you are using for this job.
        #[builder(into, default)]
        pub vpc_config: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::CustomModelVpcConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct CustomModelResult {
        /// The Amazon Resource Name (ARN) of the base model.
        pub base_model_identifier: pulumi_wasm_rust::Output<String>,
        /// The ARN of the output model.
        pub custom_model_arn: pulumi_wasm_rust::Output<String>,
        /// The custom model is encrypted at rest using this key. Specify the key ARN.
        pub custom_model_kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Name for the custom model.
        pub custom_model_name: pulumi_wasm_rust::Output<String>,
        /// The customization type. Valid values: `FINE_TUNING`, `CONTINUED_PRE_TRAINING`.
        pub customization_type: pulumi_wasm_rust::Output<String>,
        /// [Parameters](https://docs.aws.amazon.com/bedrock/latest/userguide/custom-models-hp.html) related to tuning the model.
        pub hyperparameters: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ARN of the customization job.
        pub job_arn: pulumi_wasm_rust::Output<String>,
        /// A name for the customization job.
        pub job_name: pulumi_wasm_rust::Output<String>,
        /// The status of the customization job. A successful job transitions from `InProgress` to `Completed` when the output model is ready to use.
        pub job_status: pulumi_wasm_rust::Output<String>,
        /// S3 location for the output data.
        pub output_data_config: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::CustomModelOutputDataConfig>,
        >,
        /// The Amazon Resource Name (ARN) of an IAM role that Bedrock can assume to perform tasks on your behalf.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the customization job and custom model. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::CustomModelTimeouts>,
        >,
        /// Information about the training dataset.
        pub training_data_config: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::CustomModelTrainingDataConfig>,
        >,
        /// Metrics associated with the customization job.
        pub training_metrics: pulumi_wasm_rust::Output<
            Vec<super::super::types::bedrock::CustomModelTrainingMetric>,
        >,
        /// Information about the validation dataset.
        pub validation_data_config: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::CustomModelValidationDataConfig>,
        >,
        /// The loss metric for each validator that you provided.
        pub validation_metrics: pulumi_wasm_rust::Output<
            Vec<super::super::types::bedrock::CustomModelValidationMetric>,
        >,
        /// Configuration parameters for the private Virtual Private Cloud (VPC) that contains the resources you are using for this job.
        pub vpc_config: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::CustomModelVpcConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CustomModelArgs) -> CustomModelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let base_model_identifier_binding = args.base_model_identifier.get_inner();
        let custom_model_kms_key_id_binding = args.custom_model_kms_key_id.get_inner();
        let custom_model_name_binding = args.custom_model_name.get_inner();
        let customization_type_binding = args.customization_type.get_inner();
        let hyperparameters_binding = args.hyperparameters.get_inner();
        let job_name_binding = args.job_name.get_inner();
        let output_data_config_binding = args.output_data_config.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let tags_binding = args.tags.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let training_data_config_binding = args.training_data_config.get_inner();
        let validation_data_config_binding = args.validation_data_config.get_inner();
        let vpc_config_binding = args.vpc_config.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:bedrock/customModel:CustomModel".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "baseModelIdentifier".into(),
                    value: &base_model_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "customModelKmsKeyId".into(),
                    value: &custom_model_kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "customModelName".into(),
                    value: &custom_model_name_binding,
                },
                register_interface::ObjectField {
                    name: "customizationType".into(),
                    value: &customization_type_binding,
                },
                register_interface::ObjectField {
                    name: "hyperparameters".into(),
                    value: &hyperparameters_binding,
                },
                register_interface::ObjectField {
                    name: "jobName".into(),
                    value: &job_name_binding,
                },
                register_interface::ObjectField {
                    name: "outputDataConfig".into(),
                    value: &output_data_config_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
                register_interface::ObjectField {
                    name: "trainingDataConfig".into(),
                    value: &training_data_config_binding,
                },
                register_interface::ObjectField {
                    name: "validationDataConfig".into(),
                    value: &validation_data_config_binding,
                },
                register_interface::ObjectField {
                    name: "vpcConfig".into(),
                    value: &vpc_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "baseModelIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "customModelArn".into(),
                },
                register_interface::ResultField {
                    name: "customModelKmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "customModelName".into(),
                },
                register_interface::ResultField {
                    name: "customizationType".into(),
                },
                register_interface::ResultField {
                    name: "hyperparameters".into(),
                },
                register_interface::ResultField {
                    name: "jobArn".into(),
                },
                register_interface::ResultField {
                    name: "jobName".into(),
                },
                register_interface::ResultField {
                    name: "jobStatus".into(),
                },
                register_interface::ResultField {
                    name: "outputDataConfig".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
                register_interface::ResultField {
                    name: "trainingDataConfig".into(),
                },
                register_interface::ResultField {
                    name: "trainingMetrics".into(),
                },
                register_interface::ResultField {
                    name: "validationDataConfig".into(),
                },
                register_interface::ResultField {
                    name: "validationMetrics".into(),
                },
                register_interface::ResultField {
                    name: "vpcConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CustomModelResult {
            base_model_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("baseModelIdentifier").unwrap(),
            ),
            custom_model_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customModelArn").unwrap(),
            ),
            custom_model_kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customModelKmsKeyId").unwrap(),
            ),
            custom_model_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customModelName").unwrap(),
            ),
            customization_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customizationType").unwrap(),
            ),
            hyperparameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hyperparameters").unwrap(),
            ),
            job_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jobArn").unwrap(),
            ),
            job_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jobName").unwrap(),
            ),
            job_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jobStatus").unwrap(),
            ),
            output_data_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outputDataConfig").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
            training_data_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trainingDataConfig").unwrap(),
            ),
            training_metrics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trainingMetrics").unwrap(),
            ),
            validation_data_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validationDataConfig").unwrap(),
            ),
            validation_metrics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validationMetrics").unwrap(),
            ),
            vpc_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcConfig").unwrap(),
            ),
        }
    }
}
