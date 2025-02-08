#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetConfidentialLedgerCertificateBasedSecurityPrincipal {
    /// The Ledger Role to grant this Certificate Security Principal.
    #[builder(into)]
    #[serde(rename = "ledgerRoleName")]
    pub r#ledger_role_name: Box<String>,
    /// The public key, in PEM format, of the certificate used by this identity to authenticate with the Confidential Ledger.
    #[builder(into)]
    #[serde(rename = "pemPublicKey")]
    pub r#pem_public_key: Box<String>,
}
