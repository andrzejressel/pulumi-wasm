#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CacheDirectoryLdap {
    /// The base distinguished name (DN) for the LDAP domain.
    #[builder(into)]
    #[serde(rename = "baseDn")]
    pub r#base_dn: Box<String>,
    /// A `bind` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "bind")]
    pub r#bind: Box<Option<super::super::types::hpc::CacheDirectoryLdapBind>>,
    /// The URI of the CA certificate to validate the LDAP secure connection.
    #[builder(into, default)]
    #[serde(rename = "certificateValidationUri")]
    pub r#certificate_validation_uri: Box<Option<String>>,
    /// Whether the certificate should be automatically downloaded. This can be set to `true` only when `certificate_validation_uri` is provided.
    #[builder(into, default)]
    #[serde(rename = "downloadCertificateAutomatically")]
    pub r#download_certificate_automatically: Box<Option<bool>>,
    /// Whether the LDAP connection should be encrypted?
    #[builder(into, default)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: Box<Option<bool>>,
    /// The FQDN or IP address of the LDAP server.
    #[builder(into)]
    #[serde(rename = "server")]
    pub r#server: Box<String>,
}
