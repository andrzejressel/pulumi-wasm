#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationApplicationConfigurationApplicationCodeConfigurationCodeContent {
    /// Information about the Amazon S3 bucket containing the application code.
    #[builder(into, default)]
    #[serde(rename = "s3ContentLocation")]
    pub r#s_3_content_location: Box<Option<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationApplicationCodeConfigurationCodeContentS3ContentLocation>>,
    /// The text-format code for the application.
    #[builder(into, default)]
    #[serde(rename = "textContent")]
    pub r#text_content: Box<Option<String>>,
}