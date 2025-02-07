#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsUdpOutputSettingsFecOutputSettings {
    /// The height of the FEC protection matrix.
    #[builder(into, default)]
    #[serde(rename = "columnDepth")]
    pub r#column_depth: Box<Option<i32>>,
    /// Enables column only or column and row based FEC.
    #[builder(into, default)]
    #[serde(rename = "includeFec")]
    pub r#include_fec: Box<Option<String>>,
    /// The width of the FEC protection matrix.
    #[builder(into, default)]
    #[serde(rename = "rowLength")]
    pub r#row_length: Box<Option<i32>>,
}
