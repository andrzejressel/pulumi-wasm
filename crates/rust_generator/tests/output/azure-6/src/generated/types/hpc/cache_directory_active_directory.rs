#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CacheDirectoryActiveDirectory {
    /// The NetBIOS name to assign to the HPC Cache when it joins the Active Directory domain as a server.
    #[builder(into)]
    #[serde(rename = "cacheNetbiosName")]
    pub r#cache_netbios_name: Box<String>,
    /// The primary DNS IP address used to resolve the Active Directory domain controller's FQDN.
    #[builder(into)]
    #[serde(rename = "dnsPrimaryIp")]
    pub r#dns_primary_ip: Box<String>,
    /// The secondary DNS IP address used to resolve the Active Directory domain controller's FQDN.
    #[builder(into, default)]
    #[serde(rename = "dnsSecondaryIp")]
    pub r#dns_secondary_ip: Box<Option<String>>,
    /// The fully qualified domain name of the Active Directory domain controller.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: Box<String>,
    /// The Active Directory domain's NetBIOS name.
    #[builder(into)]
    #[serde(rename = "domainNetbiosName")]
    pub r#domain_netbios_name: Box<String>,
    /// The password of the Active Directory domain administrator.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// The username of the Active Directory domain administrator.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
