#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLogDataProtectionPolicyDocumentStatementOperationAuditFindingsDestinationCloudwatchLogs {
    /// Name of the CloudWatch Log Group to send findings to.
    #[builder(into)]
    #[serde(rename = "logGroup")]
    pub r#log_group: Box<String>,
}
