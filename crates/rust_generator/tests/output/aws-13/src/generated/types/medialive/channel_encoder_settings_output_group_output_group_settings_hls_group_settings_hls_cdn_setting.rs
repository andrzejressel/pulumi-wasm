#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSetting {
    #[builder(into, default)]
    #[serde(rename = "hlsAkamaiSettings")]
    pub r#hls_akamai_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsAkamaiSettings>>,
    #[builder(into, default)]
    #[serde(rename = "hlsBasicPutSettings")]
    pub r#hls_basic_put_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsBasicPutSettings>>,
    #[builder(into, default)]
    #[serde(rename = "hlsMediaStoreSettings")]
    pub r#hls_media_store_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsMediaStoreSettings>>,
    #[builder(into, default)]
    #[serde(rename = "hlsS3Settings")]
    pub r#hls_s_3_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsS3Settings>>,
    #[builder(into, default)]
    #[serde(rename = "hlsWebdavSettings")]
    pub r#hls_webdav_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsWebdavSettings>>,
}
