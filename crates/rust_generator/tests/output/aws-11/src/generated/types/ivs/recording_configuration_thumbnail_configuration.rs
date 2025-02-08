#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RecordingConfigurationThumbnailConfiguration {
    /// Thumbnail recording mode. Valid values: `DISABLED`, `INTERVAL`.
    #[builder(into, default)]
    #[serde(rename = "recordingMode")]
    pub r#recording_mode: Box<Option<String>>,
    /// The targeted thumbnail-generation interval in seconds.
    #[builder(into, default)]
    #[serde(rename = "targetIntervalSeconds")]
    pub r#target_interval_seconds: Box<Option<i32>>,
}
