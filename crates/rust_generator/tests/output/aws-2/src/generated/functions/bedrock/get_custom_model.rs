#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_custom_model {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCustomModelArgs {
        /// Name or ARN of the custom model.
        #[builder(into)]
        pub model_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetCustomModelResult {
        /// ARN of the base model.
        pub base_model_arn: pulumi_gestalt_rust::Output<String>,
        /// Creation time of the model.
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        /// Hyperparameter values associated with this model.
        pub hyperparameters: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Job ARN associated with this model.
        pub job_arn: pulumi_gestalt_rust::Output<String>,
        /// Job name associated with this model.
        pub job_name: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of tags for the fine-tuning job.
        pub job_tags: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// ARN associated with this model.
        pub model_arn: pulumi_gestalt_rust::Output<String>,
        pub model_id: pulumi_gestalt_rust::Output<String>,
        /// The custom model is encrypted at rest using this key.
        pub model_kms_key_arn: pulumi_gestalt_rust::Output<String>,
        /// Model name associated with this model.
        pub model_name: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of tags for the model.
        pub model_tags: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output data configuration associated with this custom model.
        pub output_data_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::bedrock::GetCustomModelOutputDataConfig>,
        >,
        /// Information about the training dataset.
        pub training_data_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::bedrock::GetCustomModelTrainingDataConfig>,
        >,
        /// Metrics associated with the customization job.
        pub training_metrics: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::bedrock::GetCustomModelTrainingMetric>,
        >,
        /// Information about the validation dataset.
        pub validation_data_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::bedrock::GetCustomModelValidationDataConfig>,
        >,
        /// The loss metric for each validator that you provided.
        pub validation_metrics: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::bedrock::GetCustomModelValidationMetric>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCustomModelArgs,
    ) -> GetCustomModelResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let model_id_binding = args.model_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:bedrock/getCustomModel:getCustomModel".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "modelId".into(),
                    value: model_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCustomModelResult {
            base_model_arn: o.get_field("baseModelArn"),
            creation_time: o.get_field("creationTime"),
            hyperparameters: o.get_field("hyperparameters"),
            id: o.get_field("id"),
            job_arn: o.get_field("jobArn"),
            job_name: o.get_field("jobName"),
            job_tags: o.get_field("jobTags"),
            model_arn: o.get_field("modelArn"),
            model_id: o.get_field("modelId"),
            model_kms_key_arn: o.get_field("modelKmsKeyArn"),
            model_name: o.get_field("modelName"),
            model_tags: o.get_field("modelTags"),
            output_data_configs: o.get_field("outputDataConfigs"),
            training_data_configs: o.get_field("trainingDataConfigs"),
            training_metrics: o.get_field("trainingMetrics"),
            validation_data_configs: o.get_field("validationDataConfigs"),
            validation_metrics: o.get_field("validationMetrics"),
        }
    }
}
