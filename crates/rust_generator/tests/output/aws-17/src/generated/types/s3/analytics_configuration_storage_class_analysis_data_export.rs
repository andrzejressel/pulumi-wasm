#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AnalyticsConfigurationStorageClassAnalysisDataExport {
    /// Specifies the destination for the exported analytics data (documented below).
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Box<super::super::types::s3::AnalyticsConfigurationStorageClassAnalysisDataExportDestination>,
    /// Schema version of exported analytics data. Allowed values: `V_1`. Default value: `V_1`.
    #[builder(into, default)]
    #[serde(rename = "outputSchemaVersion")]
    pub r#output_schema_version: Box<Option<String>>,
}
