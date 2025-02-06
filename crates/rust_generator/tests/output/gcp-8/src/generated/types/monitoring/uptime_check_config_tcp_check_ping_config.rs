#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UptimeCheckConfigTcpCheckPingConfig {
    /// Number of ICMP pings. A maximum of 3 ICMP pings is currently supported.
    #[builder(into)]
    #[serde(rename = "pingsCount")]
    pub r#pings_count: Box<i32>,
}
