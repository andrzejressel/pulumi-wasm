#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputProcessingConfiguration {
    /// Describes the Lambda function that is used to preprocess the records in the stream before being processed by your application code.
    #[builder(into)]
    #[serde(rename = "inputLambdaProcessor")]
    pub r#input_lambda_processor: Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputProcessingConfigurationInputLambdaProcessor>,
}
