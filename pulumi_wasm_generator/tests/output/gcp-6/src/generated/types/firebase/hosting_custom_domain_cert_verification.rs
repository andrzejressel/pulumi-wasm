#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HostingCustomDomainCertVerification {
    /// A `TXT` record to add to your DNS records that confirms your intent to
    /// let Hosting create an SSL cert for your domain name.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "dns")]
    pub r#dns: Box<Option<super::super::types::firebase::HostingCustomDomainCertVerificationDns>>,
    /// A file to add to your existing, non-Hosting hosting service that confirms
    /// your intent to let Hosting create an SSL cert for your domain name.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "http")]
    pub r#http: Box<Option<super::super::types::firebase::HostingCustomDomainCertVerificationHttp>>,
}
