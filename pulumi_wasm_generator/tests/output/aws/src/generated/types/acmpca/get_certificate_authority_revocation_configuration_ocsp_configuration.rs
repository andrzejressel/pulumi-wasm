#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCertificateAuthorityRevocationConfigurationOcspConfiguration {
    /// Boolean value that specifies whether a custom OCSP responder is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// A CNAME specifying a customized OCSP domain.
    #[builder(into)]
    #[serde(rename = "ocspCustomCname")]
    pub r#ocsp_custom_cname: Box<String>,
}
