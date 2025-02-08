#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AadDiagnosticSettingEnabledLog {
    /// The log category for the Azure Active Directory Diagnostic.
    #[builder(into)]
    #[serde(rename = "category")]
    pub r#category: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "retentionPolicy")]
    pub r#retention_policy: Box<Option<super::super::types::monitoring::AadDiagnosticSettingEnabledLogRetentionPolicy>>,
}
