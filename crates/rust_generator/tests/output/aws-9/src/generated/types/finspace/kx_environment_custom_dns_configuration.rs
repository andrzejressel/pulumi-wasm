#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KxEnvironmentCustomDnsConfiguration {
    /// IP address of the DNS server.
    #[builder(into)]
    #[serde(rename = "customDnsServerIp")]
    pub r#custom_dns_server_ip: Box<String>,
    /// Name of the DNS server.
    #[builder(into)]
    #[serde(rename = "customDnsServerName")]
    pub r#custom_dns_server_name: Box<String>,
}
