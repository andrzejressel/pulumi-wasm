#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsFileSystemSelfManagedActiveDirectory {
    /// A list of up to two IP addresses of DNS servers or domain controllers in the self-managed AD directory. The IP addresses need to be either in the same VPC CIDR range as the file system or in the private IP version 4 (IPv4) address ranges as specified in [RFC 1918](https://tools.ietf.org/html/rfc1918).
    #[builder(into)]
    #[serde(rename = "dnsIps")]
    pub r#dns_ips: Box<Vec<String>>,
    /// The fully qualified domain name of the self-managed AD directory. For example, `corp.example.com`.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: Box<String>,
    /// The name of the domain group whose members are granted administrative privileges for the file system. Administrative privileges include taking ownership of files and folders, and setting audit controls (audit ACLs) on files and folders. The group that you specify must already exist in your domain. Defaults to `Domain Admins`.
    #[builder(into, default)]
    #[serde(rename = "fileSystemAdministratorsGroup")]
    pub r#file_system_administrators_group: Box<Option<String>>,
    /// The fully qualified distinguished name of the organizational unit within your self-managed AD directory that the Windows File Server instance will join. For example, `OU=FSx,DC=yourdomain,DC=corp,DC=com`. Only accepts OU as the direct parent of the file system. If none is provided, the FSx file system is created in the default location of your self-managed AD directory. To learn more, see [RFC 2253](https://tools.ietf.org/html/rfc2253).
    #[builder(into, default)]
    #[serde(rename = "organizationalUnitDistinguishedName")]
    pub r#organizational_unit_distinguished_name: Box<Option<String>>,
    /// The password for the service account on your self-managed AD domain that Amazon FSx will use to join to your AD domain.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// The user name for the service account on your self-managed AD domain that Amazon FSx will use to join to your AD domain.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
