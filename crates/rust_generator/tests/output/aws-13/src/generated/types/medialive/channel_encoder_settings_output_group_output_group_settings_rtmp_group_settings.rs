#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsRtmpGroupSettings {
    /// The ad marker type for this output group.
    #[builder(into, default)]
    #[serde(rename = "adMarkers")]
    pub r#ad_markers: Box<Option<Vec<String>>>,
    /// Authentication scheme to use when connecting with CDN.
    #[builder(into, default)]
    #[serde(rename = "authenticationScheme")]
    pub r#authentication_scheme: Box<Option<String>>,
    /// Controls behavior when content cache fills up.
    #[builder(into, default)]
    #[serde(rename = "cacheFullBehavior")]
    pub r#cache_full_behavior: Box<Option<String>>,
    /// Cache length in seconds, is used to calculate buffer size.
    #[builder(into, default)]
    #[serde(rename = "cacheLength")]
    pub r#cache_length: Box<Option<i32>>,
    /// Controls the types of data that passes to onCaptionInfo outputs.
    #[builder(into, default)]
    #[serde(rename = "captionData")]
    pub r#caption_data: Box<Option<String>>,
    /// Controls the behavior of the RTMP group if input becomes unavailable.
    #[builder(into, default)]
    #[serde(rename = "inputLossAction")]
    pub r#input_loss_action: Box<Option<String>>,
    /// Number of seconds to wait until a restart is initiated.
    #[builder(into, default)]
    #[serde(rename = "restartDelay")]
    pub r#restart_delay: Box<Option<i32>>,
}
