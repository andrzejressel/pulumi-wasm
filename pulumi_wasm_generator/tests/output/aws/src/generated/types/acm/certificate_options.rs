#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertificateOptions {
    /// Whether certificate details should be added to a certificate transparency log. Valid values are `ENABLED` or `DISABLED`. See https://docs.aws.amazon.com/acm/latest/userguide/acm-concepts.html#concept-transparency for more details.
    #[builder(into, default)]
    #[serde(rename = "certificateTransparencyLoggingPreference")]
    pub r#certificate_transparency_logging_preference: Box<Option<String>>,
}