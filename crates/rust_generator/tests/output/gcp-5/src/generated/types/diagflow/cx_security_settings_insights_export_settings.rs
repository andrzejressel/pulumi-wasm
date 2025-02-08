#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CxSecuritySettingsInsightsExportSettings {
    /// If enabled, we will automatically exports conversations to Insights and Insights runs its analyzers.
    #[builder(into)]
    #[serde(rename = "enableInsightsExport")]
    pub r#enable_insights_export: Box<bool>,
}
