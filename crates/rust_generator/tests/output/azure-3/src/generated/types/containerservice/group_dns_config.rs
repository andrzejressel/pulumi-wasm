#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GroupDnsConfig {
    /// A list of nameservers the containers will search out to resolve requests. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "nameservers")]
    pub r#nameservers: Box<Vec<String>>,
    /// A list of [resolver configuration options](https://man7.org/linux/man-pages/man5/resolv.conf.5.html). Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "options")]
    pub r#options: Box<Option<Vec<String>>>,
    /// A list of search domains that DNS requests will search along. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "searchDomains")]
    pub r#search_domains: Box<Option<Vec<String>>>,
}
