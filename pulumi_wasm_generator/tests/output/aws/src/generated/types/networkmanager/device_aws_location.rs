#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeviceAwsLocation {
    /// The Amazon Resource Name (ARN) of the subnet that the device is located in.
    #[builder(into, default)]
    #[serde(rename = "subnetArn")]
    pub r#subnet_arn: Box<Option<String>>,
    /// The Zone that the device is located in. Specify the ID of an Availability Zone, Local Zone, Wavelength Zone, or an Outpost.
    #[builder(into, default)]
    #[serde(rename = "zone")]
    pub r#zone: Box<Option<String>>,
}