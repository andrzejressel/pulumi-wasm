#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRegionInstanceTemplateNetworkInterfaceAccessConfig {
    /// The IP address that will be 1:1 mapped to the instance's
    /// network ip. If not given, one will be generated.
    #[builder(into)]
    #[serde(rename = "natIp")]
    pub r#nat_ip: Box<String>,
    /// The [networking tier][network-tier] used for configuring
    /// this instance template. This field can take the following values: PREMIUM or
    /// STANDARD. If this field is not specified, it is assumed to be PREMIUM.
    #[builder(into)]
    #[serde(rename = "networkTier")]
    pub r#network_tier: Box<String>,
    /// The DNS domain name for the public PTR record.The DNS domain name for the public PTR record.
    #[builder(into)]
    #[serde(rename = "publicPtrDomainName")]
    pub r#public_ptr_domain_name: Box<String>,
}
