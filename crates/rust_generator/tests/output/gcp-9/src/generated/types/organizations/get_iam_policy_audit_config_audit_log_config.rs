#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetIamPolicyAuditConfigAuditLogConfig {
    /// Specifies the identities that are exempt from these types of logging operations. Follows the same format of the `members` array for `binding`.
    #[builder(into, default)]
    #[serde(rename = "exemptedMembers")]
    pub r#exempted_members: Box<Option<Vec<String>>>,
    /// Defines the logging level. `DATA_READ`, `DATA_WRITE` and `ADMIN_READ` capture different types of events. See [the audit configuration documentation](https://cloud.google.com/resource-manager/reference/rest/Shared.Types/AuditConfig) for more details.
    #[builder(into)]
    #[serde(rename = "logType")]
    pub r#log_type: Box<String>,
}
