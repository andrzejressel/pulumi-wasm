#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServerlessSecurityConfigSamlOptions {
    /// Group attribute for this SAML integration.
    #[builder(into)]
    #[serde(rename = "groupAttribute")]
    pub r#group_attribute: Box<String>,
    /// The XML IdP metadata file generated from your identity provider.
    #[builder(into)]
    #[serde(rename = "metadata")]
    pub r#metadata: Box<String>,
    /// Session timeout, in minutes. Minimum is 5 minutes and maximum is 720 minutes (12 hours). Default is 60 minutes.
    #[builder(into)]
    #[serde(rename = "sessionTimeout")]
    pub r#session_timeout: Box<i32>,
    /// User attribute for this SAML integration.
    #[builder(into)]
    #[serde(rename = "userAttribute")]
    pub r#user_attribute: Box<String>,
}
