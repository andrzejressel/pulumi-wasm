#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AnalyticsApplicationReferenceDataSourcesSchemaRecordColumn {
    /// The Mapping reference to the data element.
    #[builder(into, default)]
    #[serde(rename = "mapping")]
    pub r#mapping: Box<Option<String>>,
    /// Name of the column.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The SQL Type of the column.
    #[builder(into)]
    #[serde(rename = "sqlType")]
    pub r#sql_type: Box<String>,
}
