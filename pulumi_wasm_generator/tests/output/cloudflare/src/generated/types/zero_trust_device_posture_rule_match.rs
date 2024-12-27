#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ZeroTrustDevicePostureRuleMatch {
    /// The platform of the device. Available values: `windows`, `mac`, `linux`, `android`, `ios`, `chromeos`.
    #[builder(into, default)]
    #[serde(rename = "platform")]
    pub r#platform: Box<Option<String>>,
}
