#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProductResponse {
    /// Availability information of the product system.
    #[builder(into)]
    #[serde(rename = "availabilityInformation")]
    pub r#availability_information: Box<super::types::AvailabilityInformationResponse>,
    /// List of configurations for the product
    #[builder(into)]
    #[serde(rename = "configurations")]
    pub r#configurations: Box<Vec<super::types::ConfigurationResponse>>,
    /// Cost information for the product system.
    #[builder(into)]
    #[serde(rename = "costInformation")]
    pub r#cost_information: Box<super::types::CostInformationResponse>,
    /// Description related to the product system.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<super::types::DescriptionResponse>,
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
}
