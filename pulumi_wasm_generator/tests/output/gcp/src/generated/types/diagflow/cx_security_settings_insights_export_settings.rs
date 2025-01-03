#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxSecuritySettingsInsightsExportSettings {
    /// If enabled, we will automatically exports conversations to Insights and Insights runs its analyzers.
    #[builder(into)]
    #[serde(rename = "enableInsightsExport")]
    pub r#enable_insights_export: Box<bool>,
}
