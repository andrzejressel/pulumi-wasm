#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsTimecodeConfig {
    /// The source for the timecode that will be associated with the events outputs.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<String>,
    /// Threshold in frames beyond which output timecode is resynchronized to the input timecode.
    #[builder(into, default)]
    #[serde(rename = "syncThreshold")]
    pub r#sync_threshold: Box<Option<i32>>,
}
