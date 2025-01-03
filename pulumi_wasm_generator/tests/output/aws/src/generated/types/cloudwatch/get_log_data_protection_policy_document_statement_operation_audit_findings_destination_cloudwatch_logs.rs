#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLogDataProtectionPolicyDocumentStatementOperationAuditFindingsDestinationCloudwatchLogs {
    /// Name of the CloudWatch Log Group to send findings to.
    #[builder(into)]
    #[serde(rename = "logGroup")]
    pub r#log_group: Box<String>,
}
