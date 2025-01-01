#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CaPoolIssuancePolicyAllowedKeyTypeEllipticCurve {
    /// The algorithm used.
    /// Possible values are: `ECDSA_P256`, `ECDSA_P384`, `EDDSA_25519`.
    #[builder(into)]
    #[serde(rename = "signatureAlgorithm")]
    pub r#signature_algorithm: Box<String>,
}
