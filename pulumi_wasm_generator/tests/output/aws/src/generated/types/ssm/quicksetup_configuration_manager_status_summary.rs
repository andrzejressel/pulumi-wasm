#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct QuicksetupConfigurationManagerStatusSummary {
    /// Current status.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
    /// When applicable, returns an informational message relevant to the current status and status type of the status summary object.
    #[builder(into)]
    #[serde(rename = "statusMessage")]
    pub r#status_message: Box<String>,
    /// Type of a status summary.
    #[builder(into)]
    #[serde(rename = "statusType")]
    pub r#status_type: Box<String>,
}
