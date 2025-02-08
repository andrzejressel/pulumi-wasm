#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AlertPolicyConditionConditionPrometheusQueryLanguage {
    /// The alerting rule name of this alert in the corresponding Prometheus
    /// configuration file.
    /// Some external tools may require this field to be populated correctly
    /// in order to refer to the original Prometheus configuration file.
    /// The rule group name and the alert name are necessary to update the
    /// relevant AlertPolicies in case the definition of the rule group changes
    /// in the future.
    /// This field is optional. If this field is not empty, then it must be a
    /// valid Prometheus label name.
    #[builder(into, default)]
    #[serde(rename = "alertRule")]
    pub r#alert_rule: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "disableMetricValidation")]
    pub r#disable_metric_validation: Box<Option<bool>>,
    /// Alerts are considered firing once their PromQL expression evaluated
    /// to be "true" for this long. Alerts whose PromQL expression was not
    /// evaluated to be "true" for long enough are considered pending. The
    /// default value is zero. Must be zero or positive.
    #[builder(into, default)]
    #[serde(rename = "duration")]
    pub r#duration: Box<Option<String>>,
    /// How often this rule should be evaluated. Must be a positive multiple
    /// of 30 seconds or missing. The default value is 30 seconds. If this
    /// PrometheusQueryLanguageCondition was generated from a Prometheus
    /// alerting rule, then this value should be taken from the enclosing
    /// rule group.
    #[builder(into, default)]
    #[serde(rename = "evaluationInterval")]
    pub r#evaluation_interval: Box<Option<String>>,
    /// Labels to add to or overwrite in the PromQL query result. Label names
    /// must be valid.
    /// Label values can be templatized by using variables. The only available
    /// variable names are the names of the labels in the PromQL result, including
    /// "__name__" and "value". "labels" may be empty. This field is intended to be
    /// used for organizing and identifying the AlertPolicy
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// The PromQL expression to evaluate. Every evaluation cycle this
    /// expression is evaluated at the current time, and all resultant time
    /// series become pending/firing alerts. This field must not be empty.
    #[builder(into)]
    #[serde(rename = "query")]
    pub r#query: Box<String>,
    /// The rule group name of this alert in the corresponding Prometheus
    /// configuration file.
    /// Some external tools may require this field to be populated correctly
    /// in order to refer to the original Prometheus configuration file.
    /// The rule group name and the alert name are necessary to update the
    /// relevant AlertPolicies in case the definition of the rule group changes
    /// in the future. This field is optional.
    #[builder(into, default)]
    #[serde(rename = "ruleGroup")]
    pub r#rule_group: Box<Option<String>>,
}
