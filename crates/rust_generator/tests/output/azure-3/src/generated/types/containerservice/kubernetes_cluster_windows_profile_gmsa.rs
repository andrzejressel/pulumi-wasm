#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KubernetesClusterWindowsProfileGmsa {
    /// Specifies the DNS server for Windows gMSA. Set this to an empty string if you have configured the DNS server in the VNet which was used to create the managed cluster.
    #[builder(into)]
    #[serde(rename = "dnsServer")]
    pub r#dns_server: Box<String>,
    /// Specifies the root domain name for Windows gMSA. Set this to an empty string if you have configured the DNS server in the VNet which was used to create the managed cluster.
    /// 
    /// > **Note:** The properties `dns_server` and `root_domain` must both either be set or unset, i.e. empty.
    #[builder(into)]
    #[serde(rename = "rootDomain")]
    pub r#root_domain: Box<String>,
}
