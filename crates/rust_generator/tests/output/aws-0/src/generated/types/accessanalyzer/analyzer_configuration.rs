#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AnalyzerConfiguration {
    /// A block that specifies the configuration of an unused access analyzer for an AWS organization or account. Documented below
    #[builder(into, default)]
    #[serde(rename = "unusedAccess")]
    pub r#unused_access: Box<Option<super::super::types::accessanalyzer::AnalyzerConfigurationUnusedAccess>>,
}
