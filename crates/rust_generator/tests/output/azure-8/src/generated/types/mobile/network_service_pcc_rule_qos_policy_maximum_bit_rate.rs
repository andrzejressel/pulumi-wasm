#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkServicePccRuleQosPolicyMaximumBitRate {
    /// Downlink bit rate. Must be a number followed by `bps`, `Kbps`, `Mbps`, `Gbps` or `Tbps`.
    #[builder(into)]
    #[serde(rename = "downlink")]
    pub r#downlink: Box<String>,
    /// Uplink bit rate. Must be a number followed by `bps`, `Kbps`, `Mbps`, `Gbps` or `Tbps`.
    #[builder(into)]
    #[serde(rename = "uplink")]
    pub r#uplink: Box<String>,
}
