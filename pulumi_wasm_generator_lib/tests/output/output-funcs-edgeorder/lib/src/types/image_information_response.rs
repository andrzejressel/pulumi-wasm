//! Image for the product

#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
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
