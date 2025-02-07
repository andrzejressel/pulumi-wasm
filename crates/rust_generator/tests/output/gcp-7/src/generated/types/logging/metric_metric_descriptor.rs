#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MetricMetricDescriptor {
    /// A concise name for the metric, which can be displayed in user interfaces. Use sentence case
    /// without an ending period, for example "Request count". This field is optional but it is
    /// recommended to be set for any metrics associated with user-visible concepts, such as Quota.
    #[builder(into, default)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<Option<String>>,
    /// The set of labels that can be used to describe a specific instance of this metric type. For
    /// example, the appengine.googleapis.com/http/server/response_latencies metric type has a label
    /// for the HTTP response code, response_code, so you can look at latencies for successful responses
    /// or just for responses that failed.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<Vec<super::super::types::logging::MetricMetricDescriptorLabel>>>,
    /// Whether the metric records instantaneous values, changes to a value, etc.
    /// Some combinations of metricKind and valueType might not be supported.
    /// For counter metrics, set this to DELTA.
    /// Possible values are: `DELTA`, `GAUGE`, `CUMULATIVE`.
    #[builder(into)]
    #[serde(rename = "metricKind")]
    pub r#metric_kind: Box<String>,
    /// The unit in which the metric value is reported. It is only applicable if the valueType is
    /// `INT64`, `DOUBLE`, or `DISTRIBUTION`. The supported units are a subset of
    /// [The Unified Code for Units of Measure](http://unitsofmeasure.org/ucum.html) standard
    #[builder(into, default)]
    #[serde(rename = "unit")]
    pub r#unit: Box<Option<String>>,
    /// Whether the measurement is an integer, a floating-point number, etc.
    /// Some combinations of metricKind and valueType might not be supported.
    /// For counter metrics, set this to INT64.
    /// Possible values are: `BOOL`, `INT64`, `DOUBLE`, `STRING`, `DISTRIBUTION`, `MONEY`.
    #[builder(into)]
    #[serde(rename = "valueType")]
    pub r#value_type: Box<String>,
}
