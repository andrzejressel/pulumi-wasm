#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainAssociationSubDomain {
    /// Branch name setting for the subdomain.
    #[builder(into)]
    #[serde(rename = "branchName")]
    pub r#branch_name: Box<String>,
    /// DNS record for the subdomain in a space-prefixed and space-delimited format (` CNAME <target>`).
    #[builder(into, default)]
    #[serde(rename = "dnsRecord")]
    pub r#dns_record: Box<Option<String>>,
    /// Prefix setting for the subdomain.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<String>,
    /// Verified status of the subdomain.
    #[builder(into, default)]
    #[serde(rename = "verified")]
    pub r#verified: Box<Option<bool>>,
}
