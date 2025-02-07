#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct Container {
    #[builder(into, default)]
    #[serde(rename = "brightness")]
    pub r#brightness: Box<Option<super::types::ContainerBrightness>>,
    #[builder(into, default)]
    #[serde(rename = "color")]
    pub r#color: Box<Option<pulumi_gestalt_rust::OneOf2<super::types::ContainerColor, String>>>,
    #[builder(into, default)]
    #[serde(rename = "material")]
    pub r#material: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Box<super::types::ContainerSize>,
}
