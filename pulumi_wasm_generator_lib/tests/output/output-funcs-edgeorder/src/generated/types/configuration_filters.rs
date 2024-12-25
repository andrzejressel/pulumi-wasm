#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ConfigurationFilters {
    /// Filters specific to product
    #[builder(into, default)]
    #[serde(rename = "filterableProperty")]
    pub r#filterable_property: Box<Option<Vec<super::types::FilterableProperty>>>,
    /// Product hierarchy information
    #[builder(into)]
    #[serde(rename = "hierarchyInformation")]
    pub r#hierarchy_information: Box<super::types::HierarchyInformation>,
}
