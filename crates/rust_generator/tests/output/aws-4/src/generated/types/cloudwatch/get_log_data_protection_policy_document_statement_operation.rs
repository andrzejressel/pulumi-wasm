#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLogDataProtectionPolicyDocumentStatementOperation {
    /// Configures the detection of sensitive data.
    #[builder(into, default)]
    #[serde(rename = "audit")]
    pub r#audit: Box<Option<super::super::types::cloudwatch::GetLogDataProtectionPolicyDocumentStatementOperationAudit>>,
    /// Configures the masking of sensitive data.
    /// 
    /// > Every policy statement must specify exactly one operation.
    #[builder(into, default)]
    #[serde(rename = "deidentify")]
    pub r#deidentify: Box<Option<super::super::types::cloudwatch::GetLogDataProtectionPolicyDocumentStatementOperationDeidentify>>,
}
