#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct Container {
    #[builder(into, default)]
    #[serde(rename = "brightness")]
    pub r#brightness: Box<Option<crate::types::ContainerBrightness>>,
    #[builder(into, default)]
    #[serde(rename = "color")]
    pub r#color: Box<Option<pulumi_wasm_provider_common::OneOf2<crate::types::ContainerColor, String>>>,
    #[builder(into, default)]
    #[serde(rename = "material")]
    pub r#material: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Box<crate::types::ContainerSize>,
}
