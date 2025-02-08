#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSetLogicalTableMapDataTransformTagColumnOperationTag {
    /// A description for a column. See column_description.
    #[builder(into, default)]
    #[serde(rename = "columnDescription")]
    pub r#column_description: Box<Option<super::super::types::quicksight::DataSetLogicalTableMapDataTransformTagColumnOperationTagColumnDescription>>,
    /// A geospatial role for a column. Valid values are `COUNTRY`, `STATE`, `COUNTY`, `CITY`, `POSTCODE`, `LONGITUDE`, and `LATITUDE`.
    #[builder(into, default)]
    #[serde(rename = "columnGeographicRole")]
    pub r#column_geographic_role: Box<Option<String>>,
}
