#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationGatewaySslProfile {
    /// The ID of the Rewrite Rule Set
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The name of the SSL Profile that is unique within this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// a `ssl_policy` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "sslPolicy")]
    pub r#ssl_policy: Box<Option<super::super::types::network::ApplicationGatewaySslProfileSslPolicy>>,
    /// The name of the Trusted Client Certificate that will be used to authenticate requests from clients.
    #[builder(into, default)]
    #[serde(rename = "trustedClientCertificateNames")]
    pub r#trusted_client_certificate_names: Box<Option<Vec<String>>>,
    /// Should client certificate issuer DN be verified? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "verifyClientCertIssuerDn")]
    pub r#verify_client_cert_issuer_dn: Box<Option<bool>>,
    /// Specify the method to check client certificate revocation status. Possible value is `OCSP`.
    #[builder(into, default)]
    #[serde(rename = "verifyClientCertificateRevocation")]
    pub r#verify_client_certificate_revocation: Box<Option<String>>,
}
