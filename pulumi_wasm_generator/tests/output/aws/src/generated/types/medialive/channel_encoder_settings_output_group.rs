#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsOutputGroup {
    /// Custom output group name defined by the user.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Settings associated with the output group. See Output Group Settings for more details.
    #[builder(into)]
    #[serde(rename = "outputGroupSettings")]
    pub r#output_group_settings: Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettings>,
    /// List of outputs. See Outputs for more details.
    #[builder(into)]
    #[serde(rename = "outputs")]
    pub r#outputs: Box<Vec<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutput>>,
}
