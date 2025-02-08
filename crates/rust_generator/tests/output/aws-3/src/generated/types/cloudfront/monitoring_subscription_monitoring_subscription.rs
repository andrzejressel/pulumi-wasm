#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct MonitoringSubscriptionMonitoringSubscription {
    /// A subscription configuration for additional CloudWatch metrics. See below.
    #[builder(into)]
    #[serde(rename = "realtimeMetricsSubscriptionConfig")]
    pub r#realtime_metrics_subscription_config: Box<super::super::types::cloudfront::MonitoringSubscriptionMonitoringSubscriptionRealtimeMetricsSubscriptionConfig>,
}
