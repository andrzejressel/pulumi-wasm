#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettingsHdr10Settings {
    /// Sets the MaxCLL value for HDR10.
    #[builder(into, default)]
    #[serde(rename = "maxCll")]
    pub r#max_cll: Box<Option<i32>>,
    /// Sets the MaxFALL value for HDR10.
    #[builder(into, default)]
    #[serde(rename = "maxFall")]
    pub r#max_fall: Box<Option<i32>>,
}
