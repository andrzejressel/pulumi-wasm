#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDeviceAwsLocation {
    /// ARN of the subnet that the device is located in.
    #[builder(into)]
    #[serde(rename = "subnetArn")]
    pub r#subnet_arn: Box<String>,
    /// Zone that the device is located in.
    #[builder(into)]
    #[serde(rename = "zone")]
    pub r#zone: Box<String>,
}
