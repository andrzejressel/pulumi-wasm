#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettings {
    /// M2TS Settings. See [M2TS Settings](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html) for more details.
    #[builder(into, default)]
    #[serde(rename = "m2tsSettings")]
    pub r#m_2_ts_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsM2TsSettings>>,
    /// Raw Settings. This can be set as an empty block.
    #[builder(into, default)]
    #[serde(rename = "rawSettings")]
    pub r#raw_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsArchiveOutputSettingsContainerSettingsRawSettings>>,
}