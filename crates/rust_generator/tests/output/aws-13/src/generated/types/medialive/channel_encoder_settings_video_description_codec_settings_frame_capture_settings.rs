#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ChannelEncoderSettingsVideoDescriptionCodecSettingsFrameCaptureSettings {
    /// The frequency at which to capture frames for inclusion in the output.
    #[builder(into, default)]
    #[serde(rename = "captureInterval")]
    pub r#capture_interval: Box<Option<i32>>,
    /// Unit for the frame capture interval.
    #[builder(into, default)]
    #[serde(rename = "captureIntervalUnits")]
    pub r#capture_interval_units: Box<Option<String>>,
}
