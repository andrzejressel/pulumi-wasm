#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DistributionLocation {
    /// The Availability Zone. Follows the format us-east-2a (case-sensitive).
    #[builder(into)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Box<String>,
    /// The AWS Region name.
    #[builder(into)]
    #[serde(rename = "regionName")]
    pub r#region_name: Box<String>,
}
