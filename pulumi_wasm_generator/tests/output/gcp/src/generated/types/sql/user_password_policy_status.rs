#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UserPasswordPolicyStatus {
    /// If true, user does not have login privileges.
    #[builder(into, default)]
    #[serde(rename = "locked")]
    pub r#locked: Box<Option<bool>>,
    /// Password expiration duration with one week grace period.
    #[builder(into, default)]
    #[serde(rename = "passwordExpirationTime")]
    pub r#password_expiration_time: Box<Option<String>>,
}
