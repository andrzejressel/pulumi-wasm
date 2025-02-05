#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsFrameCaptureGroupSettingsDestination {
    /// Reference ID for the destination.
    #[builder(into)]
    #[serde(rename = "destinationRefId")]
    pub r#destination_ref_id: Box<String>,
}
