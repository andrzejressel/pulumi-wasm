#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct HierarchyInformationResponse {
    /// Represents configuration name that uniquely identifies configuration
    #[builder(into, default)]
    #[serde(rename = "configurationName")]
    pub r#configuration_name: Box<Option<String>>,
    /// Represents product family name that uniquely identifies product family
    #[builder(into, default)]
    #[serde(rename = "productFamilyName")]
    pub r#product_family_name: Box<Option<String>>,
    /// Represents product line name that uniquely identifies product line
    #[builder(into, default)]
    #[serde(rename = "productLineName")]
    pub r#product_line_name: Box<Option<String>>,
    /// Represents product name that uniquely identifies product
    #[builder(into, default)]
    #[serde(rename = "productName")]
    pub r#product_name: Box<Option<String>>,
}
