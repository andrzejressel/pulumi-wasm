#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeviceDevice {
    /// A description for the device.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The name of the device.
    #[builder(into)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Box<String>,
    /// Amazon Web Services Internet of Things (IoT) object name.
    #[builder(into, default)]
    #[serde(rename = "iotThingName")]
    pub r#iot_thing_name: Box<Option<String>>,
}