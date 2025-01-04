#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingArchiveCdnSettings {
    /// Archive S3 Settings. See Archive S3 Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "archiveS3Settings")]
    pub r#archive_s_3_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsArchiveGroupSettingArchiveCdnSettingsArchiveS3Settings>>,
}
