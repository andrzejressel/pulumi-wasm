#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetAccountAzureFilesAuthenticationActiveDirectory {
    /// The domain GUID.
    #[builder(into)]
    #[serde(rename = "domainGuid")]
    pub r#domain_guid: Box<String>,
    /// The primary domain that the AD DNS server is authoritative for.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: Box<String>,
    /// The domain security identifier.
    #[builder(into)]
    #[serde(rename = "domainSid")]
    pub r#domain_sid: Box<String>,
    /// The name of the Active Directory forest.
    #[builder(into)]
    #[serde(rename = "forestName")]
    pub r#forest_name: Box<String>,
    /// The NetBIOS domain name.
    #[builder(into)]
    #[serde(rename = "netbiosDomainName")]
    pub r#netbios_domain_name: Box<String>,
    /// The security identifier for Azure Storage.
    #[builder(into)]
    #[serde(rename = "storageSid")]
    pub r#storage_sid: Box<String>,
}
