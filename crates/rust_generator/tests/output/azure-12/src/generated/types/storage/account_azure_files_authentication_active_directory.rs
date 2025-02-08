#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccountAzureFilesAuthenticationActiveDirectory {
    /// Specifies the domain GUID.
    #[builder(into)]
    #[serde(rename = "domainGuid")]
    pub r#domain_guid: Box<String>,
    /// Specifies the primary domain that the AD DNS server is authoritative for.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: Box<String>,
    /// Specifies the security identifier (SID). This is required when `directory_type` is set to `AD`.
    #[builder(into, default)]
    #[serde(rename = "domainSid")]
    pub r#domain_sid: Box<Option<String>>,
    /// Specifies the Active Directory forest. This is required when `directory_type` is set to `AD`.
    #[builder(into, default)]
    #[serde(rename = "forestName")]
    pub r#forest_name: Box<Option<String>>,
    /// Specifies the NetBIOS domain name. This is required when `directory_type` is set to `AD`.
    #[builder(into, default)]
    #[serde(rename = "netbiosDomainName")]
    pub r#netbios_domain_name: Box<Option<String>>,
    /// Specifies the security identifier (SID) for Azure Storage. This is required when `directory_type` is set to `AD`.
    #[builder(into, default)]
    #[serde(rename = "storageSid")]
    pub r#storage_sid: Box<Option<String>>,
}
