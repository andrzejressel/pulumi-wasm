#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetReceivedLicenseReceivedMetadata {
    /// A list of allowed operations.
    #[builder(into)]
    #[serde(rename = "allowedOperations")]
    pub r#allowed_operations: Box<Vec<String>>,
    /// Received status.
    #[builder(into)]
    #[serde(rename = "receivedStatus")]
    pub r#received_status: Box<String>,
    /// Received status reason.
    #[builder(into)]
    #[serde(rename = "receivedStatusReason")]
    pub r#received_status_reason: Box<String>,
}
