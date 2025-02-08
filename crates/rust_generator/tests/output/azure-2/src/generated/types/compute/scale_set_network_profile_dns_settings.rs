#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ScaleSetNetworkProfileDnsSettings {
    /// Specifies an array of DNS servers.
    #[builder(into)]
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Box<Vec<String>>,
}
