#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FlexibleAppVersionAutomaticScalingNetworkUtilization {
    /// Target bytes received per second.
    #[builder(into, default)]
    #[serde(rename = "targetReceivedBytesPerSecond")]
    pub r#target_received_bytes_per_second: Box<Option<i32>>,
    /// Target packets received per second.
    #[builder(into, default)]
    #[serde(rename = "targetReceivedPacketsPerSecond")]
    pub r#target_received_packets_per_second: Box<Option<i32>>,
    /// Target bytes sent per second.
    #[builder(into, default)]
    #[serde(rename = "targetSentBytesPerSecond")]
    pub r#target_sent_bytes_per_second: Box<Option<i32>>,
    /// Target packets sent per second.
    #[builder(into, default)]
    #[serde(rename = "targetSentPacketsPerSecond")]
    pub r#target_sent_packets_per_second: Box<Option<i32>>,
}
