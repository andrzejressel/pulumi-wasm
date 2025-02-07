#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AlertPolicyCondition {
    /// A condition that checks that a time series
    /// continues to receive new data points.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "conditionAbsent")]
    pub r#condition_absent: Box<Option<super::super::types::monitoring::AlertPolicyConditionConditionAbsent>>,
    /// A condition that checks for log messages matching given constraints.
    /// If set, no other conditions can be present.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "conditionMatchedLog")]
    pub r#condition_matched_log: Box<Option<super::super::types::monitoring::AlertPolicyConditionConditionMatchedLog>>,
    /// A Monitoring Query Language query that outputs a boolean stream
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "conditionMonitoringQueryLanguage")]
    pub r#condition_monitoring_query_language: Box<Option<super::super::types::monitoring::AlertPolicyConditionConditionMonitoringQueryLanguage>>,
    /// A condition type that allows alert policies to be defined using
    /// Prometheus Query Language (PromQL).
    /// The PrometheusQueryLanguageCondition message contains information
    /// from a Prometheus alerting rule and its associated rule group.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "conditionPrometheusQueryLanguage")]
    pub r#condition_prometheus_query_language: Box<Option<super::super::types::monitoring::AlertPolicyConditionConditionPrometheusQueryLanguage>>,
    /// A condition that compares a time series against a
    /// threshold.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "conditionThreshold")]
    pub r#condition_threshold: Box<Option<super::super::types::monitoring::AlertPolicyConditionConditionThreshold>>,
    /// A short name or phrase used to identify the
    /// condition in dashboards, notifications, and
    /// incidents. To avoid confusion, don't use the same
    /// display name for multiple conditions in the same
    /// policy.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<String>,
    /// (Output)
    /// The unique resource name for this condition.
    /// Its syntax is:
    /// projects/[PROJECT_ID]/alertPolicies/[POLICY_ID]/conditions/[CONDITION_ID]
    /// [CONDITION_ID] is assigned by Stackdriver Monitoring when
    /// the condition is created as part of a new or updated alerting
    /// policy.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
