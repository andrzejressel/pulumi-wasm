#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AlertPrometheusRuleGroupRule {
    /// An `action` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Option<Vec<super::super::types::monitoring::AlertPrometheusRuleGroupRuleAction>>>,
    /// Specifies the Alert rule name.
    #[builder(into, default)]
    #[serde(rename = "alert")]
    pub r#alert: Box<Option<String>>,
    /// An `alert_resolution` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "alertResolution")]
    pub r#alert_resolution: Box<Option<super::super::types::monitoring::AlertPrometheusRuleGroupRuleAlertResolution>>,
    /// Specifies a set of informational labels that can be used to store longer additional information such as alert descriptions or runbook links.
    #[builder(into, default)]
    #[serde(rename = "annotations")]
    pub r#annotations: Box<Option<std::collections::HashMap<String, String>>>,
    /// Is this rule enabled? Possible values are `true` and `false`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Specifies the Prometheus Query Language expression to evaluate. For more details see [this doc](https://prometheus.io/docs/prometheus/latest/querying/basics). Evaluate at the period given by `interval` and record the result as a new set of time series with the metric name given by `record`.
    #[builder(into)]
    #[serde(rename = "expression")]
    pub r#expression: Box<String>,
    /// Specifies the amount of time alert must be active before firing, represented in ISO 8601 duration format.
    #[builder(into, default)]
    #[serde(rename = "for")]
    pub r#for_: Box<Option<String>>,
    /// Specifies the labels to add or overwrite before storing the result.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// Specifies the recorded metrics name.
    #[builder(into, default)]
    #[serde(rename = "record")]
    pub r#record: Box<Option<String>>,
    /// Specifies the severity of the alerts fired by the rule. Possible values are between 0 and 4.
    #[builder(into, default)]
    #[serde(rename = "severity")]
    pub r#severity: Box<Option<i32>>,
}
