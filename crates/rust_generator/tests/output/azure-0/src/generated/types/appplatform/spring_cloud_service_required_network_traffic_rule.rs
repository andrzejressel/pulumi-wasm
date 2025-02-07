#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpringCloudServiceRequiredNetworkTrafficRule {
    /// The direction of required traffic. Possible values are `Inbound`, `Outbound`.
    #[builder(into, default)]
    #[serde(rename = "direction")]
    pub r#direction: Box<Option<String>>,
    /// The FQDN list of required traffic.
    #[builder(into, default)]
    #[serde(rename = "fqdns")]
    pub r#fqdns: Box<Option<Vec<String>>>,
    /// The IP list of required traffic.
    #[builder(into, default)]
    #[serde(rename = "ipAddresses")]
    pub r#ip_addresses: Box<Option<Vec<String>>>,
    /// The port of required traffic.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// The protocol of required traffic.
    #[builder(into, default)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
}
