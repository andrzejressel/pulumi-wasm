#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetApplicationGatewaySslProfile {
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The name of this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// a `ssl_policy` block as defined below.
    #[builder(into)]
    #[serde(rename = "sslPolicies")]
    pub r#ssl_policies: Box<Vec<super::super::types::network::GetApplicationGatewaySslProfileSslPolicy>>,
    /// The name of the Trusted Client Certificate that will be used to authenticate requests from clients.
    #[builder(into)]
    #[serde(rename = "trustedClientCertificateNames")]
    pub r#trusted_client_certificate_names: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "verifyClientCertificateIssuerDn")]
    pub r#verify_client_certificate_issuer_dn: Box<bool>,
    /// The method used to check client certificate revocation status.
    #[builder(into)]
    #[serde(rename = "verifyClientCertificateRevocation")]
    pub r#verify_client_certificate_revocation: Box<String>,
}
