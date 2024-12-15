//! Product line

#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ProductLineResponse {
    /// Availability information of the product system.
    #[builder(into)]
    #[serde(rename = "availabilityInformation")]
    pub r#availability_information: Box<crate::types::AvailabilityInformationResponse>,
    /// Cost information for the product system.
    #[builder(into)]
    #[serde(rename = "costInformation")]
    pub r#cost_information: Box<crate::types::CostInformationResponse>,
    /// Description related to the product system.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<crate::types::DescriptionResponse>,
    /// Display Name for the product system.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<String>,
    /// list of filters supported for a product
    #[builder(into)]
    #[serde(rename = "filterableProperties")]
    pub r#filterable_properties: Box<Vec<crate::types::FilterablePropertyResponse>>,
    /// Hierarchy information of a product.
    #[builder(into)]
    #[serde(rename = "hierarchyInformation")]
    pub r#hierarchy_information: Box<crate::types::HierarchyInformationResponse>,
    /// Image information for the product system.
    #[builder(into)]
    #[serde(rename = "imageInformation")]
    pub r#image_information: Box<Vec<crate::types::ImageInformationResponse>>,
    /// List of products in the product line
    #[builder(into)]
    #[serde(rename = "products")]
    pub r#products: Box<Vec<crate::types::ProductResponse>>,
}
