#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowDestinationFlowConfigDestinationConnectorPropertiesSalesforceErrorHandlingConfig {
    /// Name of the Amazon S3 bucket.
    #[builder(into, default)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<Option<String>>,
    /// Amazon S3 bucket prefix.
    #[builder(into, default)]
    #[serde(rename = "bucketPrefix")]
    pub r#bucket_prefix: Box<Option<String>>,
    /// If the flow should fail after the first instance of a failure when attempting to place data in the destination.
    #[builder(into, default)]
    #[serde(rename = "failOnFirstDestinationError")]
    pub r#fail_on_first_destination_error: Box<Option<bool>>,
}
