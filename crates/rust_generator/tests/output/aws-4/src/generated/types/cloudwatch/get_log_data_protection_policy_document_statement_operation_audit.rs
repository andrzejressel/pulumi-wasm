#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetLogDataProtectionPolicyDocumentStatementOperationAudit {
    /// Configures destinations to send audit findings to.
    #[builder(into)]
    #[serde(rename = "findingsDestination")]
    pub r#findings_destination: Box<super::super::types::cloudwatch::GetLogDataProtectionPolicyDocumentStatementOperationAuditFindingsDestination>,
}
