#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetAlertRuleTemplateScheduledTemplate {
    /// The description of this Sentinel Scheduled Alert Rule Template.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// The query of this Sentinel Scheduled Alert Rule Template.
    #[builder(into)]
    #[serde(rename = "query")]
    pub r#query: Box<String>,
    /// The ISO 8601 timespan duration between two consecutive queries.
    #[builder(into)]
    #[serde(rename = "queryFrequency")]
    pub r#query_frequency: Box<String>,
    /// The ISO 8601 timespan duration, which determine the time period of the data covered by the query.
    #[builder(into)]
    #[serde(rename = "queryPeriod")]
    pub r#query_period: Box<String>,
    /// The alert severity of this Sentinel Scheduled Alert Rule Template.
    #[builder(into)]
    #[serde(rename = "severity")]
    pub r#severity: Box<String>,
    /// A list of categories of attacks by which to classify the rule.
    #[builder(into)]
    #[serde(rename = "tactics")]
    pub r#tactics: Box<Vec<String>>,
    /// The alert trigger operator, combined with `trigger_threshold`, setting alert threshold of this Sentinel Scheduled Alert Rule Template.
    #[builder(into)]
    #[serde(rename = "triggerOperator")]
    pub r#trigger_operator: Box<String>,
    /// The baseline number of query results generated, combined with `trigger_operator`, setting alert threshold of this Sentinel Scheduled Alert Rule Template.
    #[builder(into)]
    #[serde(rename = "triggerThreshold")]
    pub r#trigger_threshold: Box<i32>,
}
