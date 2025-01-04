#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IndexUserTokenConfigurationsJwtTokenTypeConfiguration {
    /// The regular expression that identifies the claim. Minimum length of 1. Maximum length of 100.
    #[builder(into, default)]
    #[serde(rename = "claimRegex")]
    pub r#claim_regex: Box<Option<String>>,
    /// The group attribute field. Minimum length of 1. Maximum length of 100.
    #[builder(into, default)]
    #[serde(rename = "groupAttributeField")]
    pub r#group_attribute_field: Box<Option<String>>,
    /// The issuer of the token. Minimum length of 1. Maximum length of 65.
    #[builder(into, default)]
    #[serde(rename = "issuer")]
    pub r#issuer: Box<Option<String>>,
    /// The location of the key. Valid values are `URL` or `SECRET_MANAGER`
    #[builder(into)]
    #[serde(rename = "keyLocation")]
    pub r#key_location: Box<String>,
    /// The Amazon Resource Name (ARN) of the secret.
    #[builder(into, default)]
    #[serde(rename = "secretsManagerArn")]
    pub r#secrets_manager_arn: Box<Option<String>>,
    /// The signing key URL. Valid pattern is `^(https?|ftp|file):\/\/([^\s]*)`
    #[builder(into, default)]
    #[serde(rename = "url")]
    pub r#url: Box<Option<String>>,
    /// The user name attribute field. Minimum length of 1. Maximum length of 100.
    #[builder(into, default)]
    #[serde(rename = "userNameAttributeField")]
    pub r#user_name_attribute_field: Box<Option<String>>,
}
