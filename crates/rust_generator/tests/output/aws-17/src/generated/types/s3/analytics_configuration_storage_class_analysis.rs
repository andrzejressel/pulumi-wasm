#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AnalyticsConfigurationStorageClassAnalysis {
    /// Data export configuration (documented below).
    #[builder(into)]
    #[serde(rename = "dataExport")]
    pub r#data_export: Box<super::super::types::s3::AnalyticsConfigurationStorageClassAnalysisDataExport>,
}
