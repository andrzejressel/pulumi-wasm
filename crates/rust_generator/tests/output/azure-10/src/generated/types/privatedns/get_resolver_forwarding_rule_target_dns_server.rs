#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetResolverForwardingRuleTargetDnsServer {
    /// The DNS server IP address.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<String>,
    /// The DNS server port.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
}
