#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccountActiveDirectory {
    /// If enabled, AES encryption will be enabled for SMB communication. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "aesEncryptionEnabled")]
    pub r#aes_encryption_enabled: Box<Option<bool>>,
    /// A list of DNS server IP addresses for the Active Directory domain. Only allows `IPv4` address.
    #[builder(into)]
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Box<Vec<String>>,
    /// The name of the Active Directory domain.
    #[builder(into)]
    #[serde(rename = "domain")]
    pub r#domain: Box<String>,
    /// Name of the active directory machine.
    #[builder(into, default)]
    #[serde(rename = "kerberosAdName")]
    pub r#kerberos_ad_name: Box<Option<String>>,
    /// kdc server IP addresses for the active directory machine.
    /// 
    /// > **IMPORTANT:** If you plan on using **Kerberos** volumes, both `ad_name` and `kdc_ip` are required in order to create the volume.
    #[builder(into, default)]
    #[serde(rename = "kerberosKdcIp")]
    pub r#kerberos_kdc_ip: Box<Option<String>>,
    /// Specifies whether or not the LDAP traffic needs to be secured via TLS. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "ldapOverTlsEnabled")]
    pub r#ldap_over_tls_enabled: Box<Option<bool>>,
    /// Specifies whether or not the LDAP traffic needs to be signed. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "ldapSigningEnabled")]
    pub r#ldap_signing_enabled: Box<Option<bool>>,
    /// If enabled, NFS client local users can also (in addition to LDAP users) access the NFS volumes. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "localNfsUsersWithLdapAllowed")]
    pub r#local_nfs_users_with_ldap_allowed: Box<Option<bool>>,
    /// The Organizational Unit (OU) within Active Directory where machines will be created. If blank, defaults to `CN=Computers`.
    #[builder(into, default)]
    #[serde(rename = "organizationalUnit")]
    pub r#organizational_unit: Box<Option<String>>,
    /// The password associated with the `username`.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// When LDAP over SSL/TLS is enabled, the LDAP client is required to have a *base64 encoded Active Directory Certificate Service's self-signed root CA certificate*, this optional parameter is used only for dual protocol with LDAP user-mapping volumes. Required if `ldap_over_tls_enabled` is set to `true`.
    #[builder(into, default)]
    #[serde(rename = "serverRootCaCertificate")]
    pub r#server_root_ca_certificate: Box<Option<String>>,
    /// The Active Directory site the service will limit Domain Controller discovery to. If blank, defaults to `Default-First-Site-Name`.
    #[builder(into, default)]
    #[serde(rename = "siteName")]
    pub r#site_name: Box<Option<String>>,
    /// The NetBIOS name which should be used for the NetApp SMB Server, which will be registered as a computer account in the AD and used to mount volumes.
    #[builder(into)]
    #[serde(rename = "smbServerName")]
    pub r#smb_server_name: Box<String>,
    /// The Username of Active Directory Domain Administrator.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
