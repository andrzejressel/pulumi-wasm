#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ConfigurationResponse {
    /// Availability information of the product system.
    #[builder(into)]
    #[serde(rename = "availabilityInformation")]
    pub r#availability_information: Box<super::types::AvailabilityInformationResponse>,
    /// Cost information for the product system.
    #[builder(into)]
    #[serde(rename = "costInformation")]
    pub r#cost_information: Box<super::types::CostInformationResponse>,
    /// Description related to the product system.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<super::types::DescriptionResponse>,
    /// Dimensions of the configuration
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Box<super::types::DimensionsResponse>,
    /// Display Name for the product system.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<String>,
    /// list of filters supported for a product
    #[builder(into)]
    #[serde(rename = "filterableProperties")]
    pub r#filterable_properties: Box<Vec<super::types::FilterablePropertyResponse>>,
    /// Hierarchy information of a product.
    #[builder(into)]
    #[serde(rename = "hierarchyInformation")]
    pub r#hierarchy_information: Box<super::types::HierarchyInformationResponse>,
    /// Image information for the product system.
    #[builder(into)]
    #[serde(rename = "imageInformation")]
    pub r#image_information: Box<Vec<super::types::ImageInformationResponse>>,
    /// Specifications of the configuration
    #[builder(into)]
    #[serde(rename = "specifications")]
    pub r#specifications: Box<Vec<super::types::SpecificationResponse>>,
}
