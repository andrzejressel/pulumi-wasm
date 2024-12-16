//! Returns link related to the product

#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct LinkResponse {
    /// Type of link
    #[builder(into)]
    #[serde(rename = "linkType")]
    pub r#link_type: Box<String>,
    /// Url of the link
    #[builder(into)]
    #[serde(rename = "linkUrl")]
    pub r#link_url: Box<String>,
}
