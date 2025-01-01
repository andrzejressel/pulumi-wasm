#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertificateCertificateDescriptionX509DescriptionCaOption {
    /// When true, the "CA" in Basic Constraints extension will be set to true.
    #[builder(into, default)]
    #[serde(rename = "isCa")]
    pub r#is_ca: Box<Option<bool>>,
    /// Refers to the "path length constraint" in Basic Constraints extension. For a CA certificate, this value describes the depth of
    /// subordinate CA certificates that are allowed. If this value is less than 0, the request will fail.
    #[builder(into, default)]
    #[serde(rename = "maxIssuerPathLength")]
    pub r#max_issuer_path_length: Box<Option<i32>>,
}
