#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetInstanceTypeOfferingFilter {
    /// Name of the filter. The `location` filter depends on the top-level `location_type` argument and if not specified, defaults to the current region.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// List of one or more values for the filter.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
