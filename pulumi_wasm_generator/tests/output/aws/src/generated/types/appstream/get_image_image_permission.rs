#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetImageImagePermission {
    /// Boolean indicating if the image can be used for a fleet.
    #[builder(into)]
    #[serde(rename = "allowFleet")]
    pub r#allow_fleet: Box<bool>,
    /// indicated whether the image can be used for an image builder.
    #[builder(into)]
    #[serde(rename = "allowImageBuilder")]
    pub r#allow_image_builder: Box<bool>,
}