#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TransferAgentPoolBandwidthLimit {
    /// Bandwidth rate in megabytes per second, distributed across all the agents in the pool.
    #[builder(into)]
    #[serde(rename = "limitMbps")]
    pub r#limit_mbps: Box<String>,
}
