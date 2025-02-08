#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MetricAlarmMetricQuery {
    /// The ID of the account where the metrics are located, if this is a cross-account alarm.
    #[builder(into, default)]
    #[serde(rename = "accountId")]
    pub r#account_id: Box<Option<String>>,
    /// The math expression to be performed on the returned data, if this object is performing a math expression. This expression can use the id of the other metrics to refer to those metrics, and can also use the id of other expressions to use the result of those expressions. For more information about metric math expressions, see Metric Math Syntax and Functions in the [Amazon CloudWatch User Guide](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/using-metric-math.html#metric-math-syntax).
    #[builder(into, default)]
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    /// A short name used to tie this object to the results in the response. If you are performing math expressions on this set of data, this name represents that data and can serve as a variable in the mathematical expression. The valid characters are letters, numbers, and underscore. The first character must be a lowercase letter.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// A human-readable label for this metric or expression. This is especially useful if this is an expression, so that you know what the value represents.
    #[builder(into, default)]
    #[serde(rename = "label")]
    pub r#label: Box<Option<String>>,
    /// The metric to be returned, along with statistics, period, and units. Use this parameter only if this object is retrieving a metric and not performing a math expression on returned data.
    #[builder(into, default)]
    #[serde(rename = "metric")]
    pub r#metric: Box<Option<super::super::types::cloudwatch::MetricAlarmMetricQueryMetric>>,
    /// Granularity in seconds of returned data points.
    /// For metrics with regular resolution, valid values are any multiple of `60`.
    /// For high-resolution metrics, valid values are `1`, `5`, `10`, `30`, or any multiple of `60`.
    #[builder(into, default)]
    #[serde(rename = "period")]
    pub r#period: Box<Option<i32>>,
    /// Specify exactly one `metric_query` to be `true` to use that `metric_query` result as the alarm.
    /// 
    /// > **NOTE:**  You must specify either `metric` or `expression`. Not both.
    #[builder(into, default)]
    #[serde(rename = "returnData")]
    pub r#return_data: Box<Option<bool>>,
}
