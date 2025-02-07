#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterLoggingInfo {
    /// Configuration block for Broker Logs settings for logging info. See below.
    #[builder(into)]
    #[serde(rename = "brokerLogs")]
    pub r#broker_logs: Box<super::super::types::msk::ClusterLoggingInfoBrokerLogs>,
}
