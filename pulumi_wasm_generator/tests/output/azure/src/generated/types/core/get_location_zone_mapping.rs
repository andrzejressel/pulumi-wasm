#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLocationZoneMapping {
    /// The logical zone id for the availability zone
    #[builder(into)]
    #[serde(rename = "logicalZone")]
    pub r#logical_zone: Box<String>,
    /// The fully qualified physical zone id of availability zone to which logical zone id is mapped to
    #[builder(into)]
    #[serde(rename = "physicalZone")]
    pub r#physical_zone: Box<String>,
}