#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsGlobalConfiguration {
    /// Value to set the initial audio gain for the Live Event.
    #[builder(into, default)]
    #[serde(rename = "initialAudioGain")]
    pub r#initial_audio_gain: Box<Option<i32>>,
    /// Indicates the action to take when the current input completes (e.g. end-of-file). When switchAndLoopInputs is configured the encoder will restart at the beginning of the first input. When “none” is configured the encoder will transcode either black, a solid color, or a user specified slate images per the “Input Loss Behavior” configuration until the next input switch occurs (which is controlled through the Channel Schedule API).
    #[builder(into, default)]
    #[serde(rename = "inputEndAction")]
    pub r#input_end_action: Box<Option<String>>,
    /// Settings for system actions when input is lost. See Input Loss Behavior for more details.
    #[builder(into, default)]
    #[serde(rename = "inputLossBehavior")]
    pub r#input_loss_behavior: Box<Option<super::super::types::medialive::ChannelEncoderSettingsGlobalConfigurationInputLossBehavior>>,
    /// Indicates how MediaLive pipelines are synchronized. PIPELINE\_LOCKING - MediaLive will attempt to synchronize the output of each pipeline to the other. EPOCH\_LOCKING - MediaLive will attempt to synchronize the output of each pipeline to the Unix epoch.
    #[builder(into, default)]
    #[serde(rename = "outputLockingMode")]
    pub r#output_locking_mode: Box<Option<String>>,
    /// Indicates whether the rate of frames emitted by the Live encoder should be paced by its system clock (which optionally may be locked to another source via NTP) or should be locked to the clock of the source that is providing the input stream.
    #[builder(into, default)]
    #[serde(rename = "outputTimingSource")]
    pub r#output_timing_source: Box<Option<String>>,
    /// Adjusts video input buffer for streams with very low video framerates. This is commonly set to enabled for music channels with less than one video frame per second.
    #[builder(into, default)]
    #[serde(rename = "supportLowFramerateInputs")]
    pub r#support_low_framerate_inputs: Box<Option<String>>,
}
