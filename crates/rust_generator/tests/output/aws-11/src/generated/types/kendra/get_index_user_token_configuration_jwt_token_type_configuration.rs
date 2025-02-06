#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetIndexUserTokenConfigurationJwtTokenTypeConfiguration {
    /// Regular expression that identifies the claim.
    #[builder(into)]
    #[serde(rename = "claimRegex")]
    pub r#claim_regex: Box<String>,
    /// The group attribute field.
    #[builder(into)]
    #[serde(rename = "groupAttributeField")]
    pub r#group_attribute_field: Box<String>,
    /// Issuer of the token.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: Box<String>,
    /// Location of the key. Valid values are `URL` or `SECRET_MANAGER`
    #[builder(into)]
    #[serde(rename = "keyLocation")]
    pub r#key_location: Box<String>,
    /// ARN of the secret.
    #[builder(into)]
    #[serde(rename = "secretsManagerArn")]
    pub r#secrets_manager_arn: Box<String>,
    /// Signing key URL.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
    /// The user name attribute field.
    #[builder(into)]
    #[serde(rename = "userNameAttributeField")]
    pub r#user_name_attribute_field: Box<String>,
}
