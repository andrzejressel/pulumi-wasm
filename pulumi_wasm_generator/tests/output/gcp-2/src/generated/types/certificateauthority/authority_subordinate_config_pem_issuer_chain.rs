#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AuthoritySubordinateConfigPemIssuerChain {
    /// Expected to be in leaf-to-root order according to RFC 5246.
    #[builder(into, default)]
    #[serde(rename = "pemCertificates")]
    pub r#pem_certificates: Box<Option<Vec<String>>>,
}
