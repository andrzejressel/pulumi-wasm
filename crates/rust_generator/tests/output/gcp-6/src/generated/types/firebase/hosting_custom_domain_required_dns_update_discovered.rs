#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HostingCustomDomainRequiredDnsUpdateDiscovered {
    /// The domain name the record pertains to, e.g. `foo.bar.com.`.
    #[builder(into, default)]
    #[serde(rename = "domainName")]
    pub r#domain_name: Box<Option<String>>,
    /// Records on the domain
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "records")]
    pub r#records: Box<Option<Vec<super::super::types::firebase::HostingCustomDomainRequiredDnsUpdateDiscoveredRecord>>>,
}
