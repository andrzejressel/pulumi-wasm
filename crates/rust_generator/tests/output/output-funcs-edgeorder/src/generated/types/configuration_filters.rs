#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
