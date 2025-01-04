#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LedgerCertificateBasedSecurityPrincipal {
    /// Specifies the Ledger Role to grant this Certificate Security Principal. Possible values are `Administrator`, `Contributor` and `Reader`.
    #[builder(into)]
    #[serde(rename = "ledgerRoleName")]
    pub r#ledger_role_name: Box<String>,
    /// The public key, in PEM format, of the certificate used by this identity to authenticate with the Confidential Ledger.
    #[builder(into)]
    #[serde(rename = "pemPublicKey")]
    pub r#pem_public_key: Box<String>,
}
