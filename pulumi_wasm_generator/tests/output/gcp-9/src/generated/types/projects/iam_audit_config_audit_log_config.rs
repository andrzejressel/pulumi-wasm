#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IamAuditConfigAuditLogConfig {
    /// Identities that do not cause logging for this type of permission.  The format is the same as that for `members`.
    #[builder(into, default)]
    #[serde(rename = "exemptedMembers")]
    pub r#exempted_members: Box<Option<Vec<String>>>,
    /// Permission type for which logging is to be configured.  Must be one of `DATA_READ`, `DATA_WRITE`, or `ADMIN_READ`.
    #[builder(into)]
    #[serde(rename = "logType")]
    pub r#log_type: Box<String>,
}
