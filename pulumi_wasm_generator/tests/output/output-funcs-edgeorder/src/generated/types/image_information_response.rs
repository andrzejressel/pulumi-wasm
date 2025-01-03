#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ImageInformationResponse {
    /// Type of the image
    #[builder(into)]
    #[serde(rename = "imageType")]
    pub r#image_type: Box<String>,
    /// Url of the image
    #[builder(into)]
    #[serde(rename = "imageUrl")]
    pub r#image_url: Box<String>,
}
