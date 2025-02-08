#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DescriptionResponse {
    /// Attributes for the product system.
    #[builder(into)]
    #[serde(rename = "attributes")]
    pub r#attributes: Box<Vec<String>>,
    /// Type of description.
    #[builder(into)]
    #[serde(rename = "descriptionType")]
    pub r#description_type: Box<String>,
    /// Keywords for the product system.
    #[builder(into)]
    #[serde(rename = "keywords")]
    pub r#keywords: Box<Vec<String>>,
    /// Links for the product system.
    #[builder(into)]
    #[serde(rename = "links")]
    pub r#links: Box<Vec<super::types::LinkResponse>>,
    /// Long description of the product system.
    #[builder(into)]
    #[serde(rename = "longDescription")]
    pub r#long_description: Box<String>,
    /// Short description of the product system.
    #[builder(into)]
    #[serde(rename = "shortDescription")]
    pub r#short_description: Box<String>,
}
