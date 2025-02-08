#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertificateManaged {
    /// (Output)
    /// Detailed state of the latest authorization attempt for each domain
    /// specified for this Managed Certificate.
    /// Structure is documented below.
    /// 
    /// 
    /// <a name="nested_provisioning_issue"></a>The `provisioning_issue` block contains:
    #[builder(into, default)]
    #[serde(rename = "authorizationAttemptInfos")]
    pub r#authorization_attempt_infos: Box<Option<Vec<super::super::types::certificatemanager::CertificateManagedAuthorizationAttemptInfo>>>,
    /// Authorizations that will be used for performing domain authorization. Either issuanceConfig or dnsAuthorizations should be specificed, but not both.
    #[builder(into, default)]
    #[serde(rename = "dnsAuthorizations")]
    pub r#dns_authorizations: Box<Option<Vec<String>>>,
    /// The domains for which a managed SSL certificate will be generated.
    /// Wildcard domains are only supported with DNS challenge resolution
    #[builder(into, default)]
    #[serde(rename = "domains")]
    pub r#domains: Box<Option<Vec<String>>>,
    /// The resource name for a CertificateIssuanceConfig used to configure private PKI certificates in the format projects/*/locations/*/certificateIssuanceConfigs/*.
    /// If this field is not set, the certificates will instead be publicly signed as documented at https://cloud.google.com/load-balancing/docs/ssl-certificates/google-managed-certs#caa.
    /// Either issuanceConfig or dnsAuthorizations should be specificed, but not both.
    #[builder(into, default)]
    #[serde(rename = "issuanceConfig")]
    pub r#issuance_config: Box<Option<String>>,
    /// (Output)
    /// Information about issues with provisioning this Managed Certificate.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "provisioningIssues")]
    pub r#provisioning_issues: Box<Option<Vec<super::super::types::certificatemanager::CertificateManagedProvisioningIssue>>>,
    /// (Output)
    /// State of the domain for managed certificate issuance.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
}
