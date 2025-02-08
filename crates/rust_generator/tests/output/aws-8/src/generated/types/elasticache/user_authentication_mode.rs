#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct UserAuthenticationMode {
    #[builder(into, default)]
    #[serde(rename = "passwordCount")]
    pub r#password_count: Box<Option<i32>>,
    /// Specifies the passwords to use for authentication if `type` is set to `password`.
    #[builder(into, default)]
    #[serde(rename = "passwords")]
    pub r#passwords: Box<Option<Vec<String>>>,
    /// Specifies the authentication type. Possible options are: `password`, `no-password-required` or `iam`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
