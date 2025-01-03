#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConfigurationSetTrackingOptions {
    /// Custom subdomain that is used to redirect email recipients to the Amazon SES event tracking domain.
    #[builder(into, default)]
    #[serde(rename = "customRedirectDomain")]
    pub r#custom_redirect_domain: Box<Option<String>>,
}
