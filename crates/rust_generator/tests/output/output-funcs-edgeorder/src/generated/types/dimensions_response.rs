#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DimensionsResponse {
    /// Depth of the device.
    #[builder(into)]
    #[serde(rename = "depth")]
    pub r#depth: Box<f64>,
    /// Height of the device.
    #[builder(into)]
    #[serde(rename = "height")]
    pub r#height: Box<f64>,
    /// Length of the device.
    #[builder(into)]
    #[serde(rename = "length")]
    pub r#length: Box<f64>,
    /// Unit for the dimensions of length, height and width.
    #[builder(into)]
    #[serde(rename = "lengthHeightUnit")]
    pub r#length_height_unit: Box<String>,
    /// Weight of the device.
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: Box<f64>,
    /// Unit for the dimensions of weight.
    #[builder(into)]
    #[serde(rename = "weightUnit")]
    pub r#weight_unit: Box<String>,
    /// Width of the device.
    #[builder(into)]
    #[serde(rename = "width")]
    pub r#width: Box<f64>,
}
