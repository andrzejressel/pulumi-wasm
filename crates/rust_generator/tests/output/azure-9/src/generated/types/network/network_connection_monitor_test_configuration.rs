#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkConnectionMonitorTestConfiguration {
    /// A `http_configuration` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "httpConfiguration")]
    pub r#http_configuration: Box<Option<super::super::types::network::NetworkConnectionMonitorTestConfigurationHttpConfiguration>>,
    /// A `icmp_configuration` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "icmpConfiguration")]
    pub r#icmp_configuration: Box<Option<super::super::types::network::NetworkConnectionMonitorTestConfigurationIcmpConfiguration>>,
    /// The name of test configuration for the Network Connection Monitor.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The preferred IP version which is used in the test evaluation. Possible values are `IPv4` and `IPv6`.
    #[builder(into, default)]
    #[serde(rename = "preferredIpVersion")]
    pub r#preferred_ip_version: Box<Option<String>>,
    /// The protocol used to evaluate tests. Possible values are `Tcp`, `Http` and `Icmp`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    /// A `success_threshold` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "successThreshold")]
    pub r#success_threshold: Box<Option<super::super::types::network::NetworkConnectionMonitorTestConfigurationSuccessThreshold>>,
    /// A `tcp_configuration` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "tcpConfiguration")]
    pub r#tcp_configuration: Box<Option<super::super::types::network::NetworkConnectionMonitorTestConfigurationTcpConfiguration>>,
    /// The time interval in seconds at which the test evaluation will happen. Defaults to `60`.
    #[builder(into, default)]
    #[serde(rename = "testFrequencyInSeconds")]
    pub r#test_frequency_in_seconds: Box<Option<i32>>,
}
