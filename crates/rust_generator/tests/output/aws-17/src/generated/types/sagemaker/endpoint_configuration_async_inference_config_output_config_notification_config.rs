#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointConfigurationAsyncInferenceConfigOutputConfigNotificationConfig {
    /// Amazon SNS topic to post a notification to when inference fails. If no topic is provided, no notification is sent on failure.
    #[builder(into, default)]
    #[serde(rename = "errorTopic")]
    pub r#error_topic: Box<Option<String>>,
    /// The Amazon SNS topics where you want the inference response to be included. Valid values are `SUCCESS_NOTIFICATION_TOPIC` and `ERROR_NOTIFICATION_TOPIC`.
    #[builder(into, default)]
    #[serde(rename = "includeInferenceResponseIns")]
    pub r#include_inference_response_ins: Box<Option<Vec<String>>>,
    /// Amazon SNS topic to post a notification to when inference completes successfully. If no topic is provided, no notification is sent on success.
    #[builder(into, default)]
    #[serde(rename = "successTopic")]
    pub r#success_topic: Box<Option<String>>,
}
