#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AuthConfigDecryptedCredentialAuthToken {
    /// The token for the auth type.
    #[builder(into, default)]
    #[serde(rename = "token")]
    pub r#token: Box<Option<String>>,
    /// Authentication type, e.g. "Basic", "Bearer", etc.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
