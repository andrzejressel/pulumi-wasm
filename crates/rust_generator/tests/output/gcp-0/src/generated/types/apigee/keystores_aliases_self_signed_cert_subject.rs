#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KeystoresAliasesSelfSignedCertSubject {
    /// Common name of the organization. Maximum length is 64 characters.
    #[builder(into, default)]
    #[serde(rename = "commonName")]
    pub r#common_name: Box<Option<String>>,
    /// Two-letter country code. Example, IN for India, US for United States of America.
    #[builder(into, default)]
    #[serde(rename = "countryCode")]
    pub r#country_code: Box<Option<String>>,
    /// Email address. Max 255 characters.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "email")]
    pub r#email: Box<Option<String>>,
    /// City or town name. Maximum length is 128 characters.
    #[builder(into, default)]
    #[serde(rename = "locality")]
    pub r#locality: Box<Option<String>>,
    /// Organization name. Maximum length is 64 characters.
    #[builder(into, default)]
    #[serde(rename = "org")]
    pub r#org: Box<Option<String>>,
    /// Organization team name. Maximum length is 64 characters.
    #[builder(into, default)]
    #[serde(rename = "orgUnit")]
    pub r#org_unit: Box<Option<String>>,
    /// State or district name. Maximum length is 128 characters.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
}
