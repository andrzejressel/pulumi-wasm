#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MonitoringSubscriptionMonitoringSubscription {
    /// A subscription configuration for additional CloudWatch metrics. See below.
    #[builder(into)]
    #[serde(rename = "realtimeMetricsSubscriptionConfig")]
    pub r#realtime_metrics_subscription_config: Box<super::super::types::cloudfront::MonitoringSubscriptionMonitoringSubscriptionRealtimeMetricsSubscriptionConfig>,
}
