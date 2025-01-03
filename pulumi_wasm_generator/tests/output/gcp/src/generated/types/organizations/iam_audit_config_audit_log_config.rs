#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IamAuditConfigAuditLogConfig {
    /// Identities that do not cause logging for this type of permission.
    /// Each entry can have one of the following values:
    /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
    /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
    /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
    /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
    #[builder(into, default)]
    #[serde(rename = "exemptedMembers")]
    pub r#exempted_members: Box<Option<Vec<String>>>,
    /// Permission type for which logging is to be configured.  Must be one of `DATA_READ`, `DATA_WRITE`, or `ADMIN_READ`.
    #[builder(into)]
    #[serde(rename = "logType")]
    pub r#log_type: Box<String>,
}
