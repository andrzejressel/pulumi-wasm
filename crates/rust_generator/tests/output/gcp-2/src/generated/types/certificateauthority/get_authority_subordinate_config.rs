#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAuthoritySubordinateConfig {
    /// This can refer to a CertificateAuthority that was used to create a
    /// subordinate CertificateAuthority. This field is used for information
    /// and usability purposes only. The resource name is in the format
    /// 'projects/*/locations/*/caPools/*/certificateAuthorities/*'.
    #[builder(into)]
    #[serde(rename = "certificateAuthority")]
    pub r#certificate_authority: Box<String>,
    /// Contains the PEM certificate chain for the issuers of this CertificateAuthority,
    /// but not pem certificate for this CA itself.
    #[builder(into)]
    #[serde(rename = "pemIssuerChains")]
    pub r#pem_issuer_chains: Box<Vec<super::super::types::certificateauthority::GetAuthoritySubordinateConfigPemIssuerChain>>,
}
