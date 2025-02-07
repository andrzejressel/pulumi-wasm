#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MultiplexProgramMultiplexProgramSettingsVideoSettingsStatmuxSettings {
    /// Maximum bitrate.
    #[builder(into, default)]
    #[serde(rename = "maximumBitrate")]
    pub r#maximum_bitrate: Box<Option<i32>>,
    /// Minimum bitrate.
    #[builder(into, default)]
    #[serde(rename = "minimumBitrate")]
    pub r#minimum_bitrate: Box<Option<i32>>,
    /// Priority value.
    #[builder(into, default)]
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
}
