#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InvocationLoggingConfigurationLoggingConfig {
    /// CloudWatch logging configuration.
    #[builder(into, default)]
    #[serde(rename = "cloudwatchConfig")]
    pub r#cloudwatch_config: Box<Option<super::super::types::bedrockmodel::InvocationLoggingConfigurationLoggingConfigCloudwatchConfig>>,
    /// Set to include embeddings data in the log delivery.
    #[builder(into)]
    #[serde(rename = "embeddingDataDeliveryEnabled")]
    pub r#embedding_data_delivery_enabled: Box<bool>,
    /// Set to include image data in the log delivery.
    #[builder(into)]
    #[serde(rename = "imageDataDeliveryEnabled")]
    pub r#image_data_delivery_enabled: Box<bool>,
    /// S3 configuration for storing log data.
    #[builder(into, default)]
    #[serde(rename = "s3Config")]
    pub r#s_3_config: Box<Option<super::super::types::bedrockmodel::InvocationLoggingConfigurationLoggingConfigS3Config>>,
    /// Set to include text data in the log delivery.
    #[builder(into)]
    #[serde(rename = "textDataDeliveryEnabled")]
    pub r#text_data_delivery_enabled: Box<bool>,
}
