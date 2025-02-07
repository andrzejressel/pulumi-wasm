#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SdkvoiceGlobalSettingsVoiceConnector {
    /// The S3 bucket that stores the Voice Connector's call detail records.
    #[builder(into, default)]
    #[serde(rename = "cdrBucket")]
    pub r#cdr_bucket: Box<Option<String>>,
}
