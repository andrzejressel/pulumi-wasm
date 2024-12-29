#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConfigurationSetTrackingOptions {
    /// The domain to use for tracking open and click events.
    #[builder(into)]
    #[serde(rename = "customRedirectDomain")]
    pub r#custom_redirect_domain: Box<String>,
}
