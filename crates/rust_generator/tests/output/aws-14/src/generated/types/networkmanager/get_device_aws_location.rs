#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
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
