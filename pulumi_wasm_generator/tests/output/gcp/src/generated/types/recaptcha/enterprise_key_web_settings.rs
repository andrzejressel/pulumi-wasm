#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EnterpriseKeyWebSettings {
    /// If set to true, it means allowed_domains will not be enforced.
    #[builder(into, default)]
    #[serde(rename = "allowAllDomains")]
    pub r#allow_all_domains: Box<Option<bool>>,
    /// If set to true, the key can be used on AMP (Accelerated Mobile Pages) websites. This is supported only for the SCORE integration type.
    #[builder(into, default)]
    #[serde(rename = "allowAmpTraffic")]
    pub r#allow_amp_traffic: Box<Option<bool>>,
    /// Domains or subdomains of websites allowed to use the key. All subdomains of an allowed domain are automatically allowed. A valid domain requires a host and must not include any path, port, query or fragment. Examples: 'example.com' or 'subdomain.example.com'
    #[builder(into, default)]
    #[serde(rename = "allowedDomains")]
    pub r#allowed_domains: Box<Option<Vec<String>>>,
    /// Settings for the frequency and difficulty at which this key triggers captcha challenges. This should only be specified for IntegrationTypes CHECKBOX and INVISIBLE. Possible values: CHALLENGE_SECURITY_PREFERENCE_UNSPECIFIED, USABILITY, BALANCE, SECURITY
    #[builder(into, default)]
    #[serde(rename = "challengeSecurityPreference")]
    pub r#challenge_security_preference: Box<Option<String>>,
    /// Required. Describes how this key is integrated with the website. Possible values: SCORE, CHECKBOX, INVISIBLE
    #[builder(into)]
    #[serde(rename = "integrationType")]
    pub r#integration_type: Box<String>,
}
