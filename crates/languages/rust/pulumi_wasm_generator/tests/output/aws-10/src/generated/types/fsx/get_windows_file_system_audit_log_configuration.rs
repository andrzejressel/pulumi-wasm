#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetWindowsFileSystemAuditLogConfiguration {
    #[builder(into)]
    #[serde(rename = "auditLogDestination")]
    pub r#audit_log_destination: Box<String>,
    #[builder(into)]
    #[serde(rename = "fileAccessAuditLogLevel")]
    pub r#file_access_audit_log_level: Box<String>,
    #[builder(into)]
    #[serde(rename = "fileShareAccessAuditLogLevel")]
    pub r#file_share_access_audit_log_level: Box<String>,
}
