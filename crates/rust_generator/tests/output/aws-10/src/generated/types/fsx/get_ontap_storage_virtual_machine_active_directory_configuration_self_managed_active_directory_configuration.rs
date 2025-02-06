#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetOntapStorageVirtualMachineActiveDirectoryConfigurationSelfManagedActiveDirectoryConfiguration {
    /// A list of up to three IP addresses of DNS servers or domain controllers in the self-managed AD directory.
    #[builder(into)]
    #[serde(rename = "dnsIps")]
    pub r#dns_ips: Box<Vec<String>>,
    /// The fully qualified domain name of the self-managed AD directory.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: Box<String>,
    /// The name of the domain group whose members have administrative privileges for the FSx file system.
    #[builder(into)]
    #[serde(rename = "fileSystemAdministratorsGroup")]
    pub r#file_system_administrators_group: Box<String>,
    /// The fully qualified distinguished name of the organizational unit within the self-managed AD directory to which the Windows File Server or ONTAP storage virtual machine (SVM) instance is joined.
    #[builder(into)]
    #[serde(rename = "organizationalUnitDistinguishedName")]
    pub r#organizational_unit_distinguished_name: Box<String>,
    /// The user name for the service account on your self-managed AD domain that FSx uses to join to your AD domain.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
