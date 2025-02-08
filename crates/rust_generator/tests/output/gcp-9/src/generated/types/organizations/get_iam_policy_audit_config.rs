#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetIamPolicyAuditConfig {
    /// A nested block that defines the operations you'd like to log.
    #[builder(into)]
    #[serde(rename = "auditLogConfigs")]
    pub r#audit_log_configs: Box<Vec<super::super::types::organizations::GetIamPolicyAuditConfigAuditLogConfig>>,
    /// Defines a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}
