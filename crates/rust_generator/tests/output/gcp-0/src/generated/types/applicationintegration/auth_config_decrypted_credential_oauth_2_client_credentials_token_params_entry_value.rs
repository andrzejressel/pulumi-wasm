#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AuthConfigDecryptedCredentialOauth2ClientCredentialsTokenParamsEntryValue {
    /// Passing a literal value
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "literalValue")]
    pub r#literal_value: Box<Option<super::super::types::applicationintegration::AuthConfigDecryptedCredentialOauth2ClientCredentialsTokenParamsEntryValueLiteralValue>>,
}
