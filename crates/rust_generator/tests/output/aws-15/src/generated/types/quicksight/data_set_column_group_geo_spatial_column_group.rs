#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSetColumnGroupGeoSpatialColumnGroup {
    /// Columns in this hierarchy.
    #[builder(into)]
    #[serde(rename = "columns")]
    pub r#columns: Box<Vec<String>>,
    /// Country code. Valid values are `US`.
    #[builder(into)]
    #[serde(rename = "countryCode")]
    pub r#country_code: Box<String>,
    /// A display name for the hierarchy.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
