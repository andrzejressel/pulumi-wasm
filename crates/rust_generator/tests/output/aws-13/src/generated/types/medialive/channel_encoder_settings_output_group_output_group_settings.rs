#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettings {
    /// Archive group settings. See Archive Group Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "archiveGroupSettings")]
    pub r#archive_group_settings: Box<Option<Vec<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSetting>>>,
    #[builder(into, default)]
    #[serde(rename = "frameCaptureGroupSettings")]
    pub r#frame_capture_group_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettings>>,
    #[builder(into, default)]
    #[serde(rename = "hlsGroupSettings")]
    pub r#hls_group_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettings>>,
    /// Media package group settings. See Media Package Group Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "mediaPackageGroupSettings")]
    pub r#media_package_group_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsMediaPackageGroupSettings>>,
    #[builder(into, default)]
    #[serde(rename = "msSmoothGroupSettings")]
    pub r#ms_smooth_group_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsMsSmoothGroupSettings>>,
    #[builder(into, default)]
    #[serde(rename = "multiplexGroupSettings")]
    pub r#multiplex_group_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsMultiplexGroupSettings>>,
    /// RTMP group settings. See RTMP Group Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "rtmpGroupSettings")]
    pub r#rtmp_group_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsRtmpGroupSettings>>,
    #[builder(into, default)]
    #[serde(rename = "udpGroupSettings")]
    pub r#udp_group_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsUdpGroupSettings>>,
}
