#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AmiFromInstanceEphemeralBlockDevice {
    /// Path at which the device is exposed to created instances.
    #[builder(into, default)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Box<Option<String>>,
    /// Name for the ephemeral device, of the form "ephemeralN" where
    /// *N* is a volume number starting from zero.
    #[builder(into, default)]
    #[serde(rename = "virtualName")]
    pub r#virtual_name: Box<Option<String>>,
}