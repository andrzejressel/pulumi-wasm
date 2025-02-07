#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AuthConfigDecryptedCredentialOauth2ClientCredentialsTokenParamsEntry {
    /// Key of the map entry.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "key")]
    pub r#key: Box<Option<super::super::types::applicationintegration::AuthConfigDecryptedCredentialOauth2ClientCredentialsTokenParamsEntryKey>>,
    /// Value of the map entry.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<super::super::types::applicationintegration::AuthConfigDecryptedCredentialOauth2ClientCredentialsTokenParamsEntryValue>>,
}
