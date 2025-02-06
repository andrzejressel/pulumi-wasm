#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCertificatesCertificateManaged {
    /// Detailed state of the latest authorization attempt for each domain
    /// specified for this Managed Certificate.
    #[builder(into)]
    #[serde(rename = "authorizationAttemptInfos")]
    pub r#authorization_attempt_infos: Box<Vec<super::super::types::certificatemanager::GetCertificatesCertificateManagedAuthorizationAttemptInfo>>,
    /// Authorizations that will be used for performing domain authorization. Either issuanceConfig or dnsAuthorizations should be specificed, but not both.
    #[builder(into)]
    #[serde(rename = "dnsAuthorizations")]
    pub r#dns_authorizations: Box<Vec<String>>,
    /// The domains for which a managed SSL certificate will be generated.
    /// Wildcard domains are only supported with DNS challenge resolution
    #[builder(into)]
    #[serde(rename = "domains")]
    pub r#domains: Box<Vec<String>>,
    /// The resource name for a CertificateIssuanceConfig used to configure private PKI certificates in the format projects/*/locations/*/certificateIssuanceConfigs/*.
    /// If this field is not set, the certificates will instead be publicly signed as documented at https://cloud.google.com/load-balancing/docs/ssl-certificates/google-managed-certs#caa.
    /// Either issuanceConfig or dnsAuthorizations should be specificed, but not both.
    #[builder(into)]
    #[serde(rename = "issuanceConfig")]
    pub r#issuance_config: Box<String>,
    /// Information about issues with provisioning this Managed Certificate.
    #[builder(into)]
    #[serde(rename = "provisioningIssues")]
    pub r#provisioning_issues: Box<Vec<super::super::types::certificatemanager::GetCertificatesCertificateManagedProvisioningIssue>>,
    /// A state of this Managed Certificate.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
}
