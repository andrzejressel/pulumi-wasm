#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDedicatedIpPoolDedicatedIp {
    /// IPv4 address.
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
    /// Indicates how complete the dedicated IP warm-up process is. When this value equals `1`, the address has completed the warm-up process and is ready for use.
    #[builder(into)]
    #[serde(rename = "warmupPercentage")]
    pub r#warmup_percentage: Box<i32>,
    /// The warm-up status of a dedicated IP address. Valid values: `IN_PROGRESS`, `DONE`.
    #[builder(into)]
    #[serde(rename = "warmupStatus")]
    pub r#warmup_status: Box<String>,
}
