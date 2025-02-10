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
///       function: aws:bedrockfoundation:getModel
///       arguments:
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod custom_model {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomModelArgs {
        /// The Amazon Resource Name (ARN) of the base model.
        #[builder(into)]
        pub base_model_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The custom model is encrypted at rest using this key. Specify the key ARN.
        #[builder(into, default)]
        pub custom_model_kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name for the custom model.
        #[builder(into)]
        pub custom_model_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The customization type. Valid values: `FINE_TUNING`, `CONTINUED_PRE_TRAINING`.
        #[builder(into, default)]
        pub customization_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// [Parameters](https://docs.aws.amazon.com/bedrock/latest/userguide/custom-models-hp.html) related to tuning the model.
        #[builder(into)]
        pub hyperparameters: pulumi_gestalt_rust::InputOrOutput<
            std::collections::HashMap<String, String>,
        >,
        /// A name for the customization job.
        #[builder(into)]
        pub job_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// S3 location for the output data.
        #[builder(into, default)]
        pub output_data_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bedrock::CustomModelOutputDataConfig>,
        >,
        /// The Amazon Resource Name (ARN) of an IAM role that Bedrock can assume to perform tasks on your behalf.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the customization job and custom model. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bedrock::CustomModelTimeouts>,
        >,
        /// Information about the training dataset.
        #[builder(into, default)]
        pub training_data_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bedrock::CustomModelTrainingDataConfig>,
        >,
        /// Information about the validation dataset.
        #[builder(into, default)]
        pub validation_data_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bedrock::CustomModelValidationDataConfig>,
        >,
        /// Configuration parameters for the private Virtual Private Cloud (VPC) that contains the resources you are using for this job.
        #[builder(into, default)]
        pub vpc_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bedrock::CustomModelVpcConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct CustomModelResult {
        /// The Amazon Resource Name (ARN) of the base model.
        pub base_model_identifier: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the output model.
        pub custom_model_arn: pulumi_gestalt_rust::Output<String>,
        /// The custom model is encrypted at rest using this key. Specify the key ARN.
        pub custom_model_kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name for the custom model.
        pub custom_model_name: pulumi_gestalt_rust::Output<String>,
        /// The customization type. Valid values: `FINE_TUNING`, `CONTINUED_PRE_TRAINING`.
        pub customization_type: pulumi_gestalt_rust::Output<String>,
        /// [Parameters](https://docs.aws.amazon.com/bedrock/latest/userguide/custom-models-hp.html) related to tuning the model.
        pub hyperparameters: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ARN of the customization job.
        pub job_arn: pulumi_gestalt_rust::Output<String>,
        /// A name for the customization job.
        pub job_name: pulumi_gestalt_rust::Output<String>,
        /// The status of the customization job. A successful job transitions from `InProgress` to `Completed` when the output model is ready to use.
        pub job_status: pulumi_gestalt_rust::Output<String>,
        /// S3 location for the output data.
        pub output_data_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::bedrock::CustomModelOutputDataConfig>,
        >,
        /// The Amazon Resource Name (ARN) of an IAM role that Bedrock can assume to perform tasks on your behalf.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the customization job and custom model. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::bedrock::CustomModelTimeouts>,
        >,
        /// Information about the training dataset.
        pub training_data_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::bedrock::CustomModelTrainingDataConfig>,
        >,
        /// Metrics associated with the customization job.
        pub training_metrics: pulumi_gestalt_rust::Output<
            Vec<super::super::types::bedrock::CustomModelTrainingMetric>,
        >,
        /// Information about the validation dataset.
        pub validation_data_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::bedrock::CustomModelValidationDataConfig>,
        >,
        /// The loss metric for each validator that you provided.
        pub validation_metrics: pulumi_gestalt_rust::Output<
            Vec<super::super::types::bedrock::CustomModelValidationMetric>,
        >,
        /// Configuration parameters for the private Virtual Private Cloud (VPC) that contains the resources you are using for this job.
        pub vpc_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::bedrock::CustomModelVpcConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CustomModelArgs,
    ) -> CustomModelResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let base_model_identifier_binding = args
            .base_model_identifier
            .get_output(context);
        let custom_model_kms_key_id_binding = args
            .custom_model_kms_key_id
            .get_output(context);
        let custom_model_name_binding = args.custom_model_name.get_output(context);
        let customization_type_binding = args.customization_type.get_output(context);
        let hyperparameters_binding = args.hyperparameters.get_output(context);
        let job_name_binding = args.job_name.get_output(context);
        let output_data_config_binding = args.output_data_config.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let training_data_config_binding = args.training_data_config.get_output(context);
        let validation_data_config_binding = args
            .validation_data_config
            .get_output(context);
        let vpc_config_binding = args.vpc_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:bedrock/customModel:CustomModel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "baseModelIdentifier".into(),
                    value: base_model_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customModelKmsKeyId".into(),
                    value: custom_model_kms_key_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customModelName".into(),
                    value: custom_model_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customizationType".into(),
                    value: customization_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hyperparameters".into(),
                    value: hyperparameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "jobName".into(),
                    value: job_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outputDataConfig".into(),
                    value: output_data_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: role_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trainingDataConfig".into(),
                    value: training_data_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validationDataConfig".into(),
                    value: validation_data_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcConfig".into(),
                    value: vpc_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CustomModelResult {
            base_model_identifier: o.get_field("baseModelIdentifier"),
            custom_model_arn: o.get_field("customModelArn"),
            custom_model_kms_key_id: o.get_field("customModelKmsKeyId"),
            custom_model_name: o.get_field("customModelName"),
            customization_type: o.get_field("customizationType"),
            hyperparameters: o.get_field("hyperparameters"),
            job_arn: o.get_field("jobArn"),
            job_name: o.get_field("jobName"),
            job_status: o.get_field("jobStatus"),
            output_data_config: o.get_field("outputDataConfig"),
            role_arn: o.get_field("roleArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
            training_data_config: o.get_field("trainingDataConfig"),
            training_metrics: o.get_field("trainingMetrics"),
            validation_data_config: o.get_field("validationDataConfig"),
            validation_metrics: o.get_field("validationMetrics"),
            vpc_config: o.get_field("vpcConfig"),
        }
    }
}
