#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AadDiagnosticSettingEnabledLog {
    /// The log category for the Azure Active Directory Diagnostic.
    #[builder(into)]
    #[serde(rename = "category")]
    pub r#category: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "retentionPolicy")]
    pub r#retention_policy: Box<Option<super::super::types::monitoring::AadDiagnosticSettingEnabledLogRetentionPolicy>>,
}