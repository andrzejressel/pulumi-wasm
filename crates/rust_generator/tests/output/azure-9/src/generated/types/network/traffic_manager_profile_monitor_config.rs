#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TrafficManagerProfileMonitorConfig {
    /// One or more `custom_header` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "customHeaders")]
    pub r#custom_headers: Box<Option<Vec<super::super::types::network::TrafficManagerProfileMonitorConfigCustomHeader>>>,
    /// A list of status code ranges in the format of `100-101`.
    #[builder(into, default)]
    #[serde(rename = "expectedStatusCodeRanges")]
    pub r#expected_status_code_ranges: Box<Option<Vec<String>>>,
    /// The interval used to check the endpoint health from a Traffic Manager probing agent. You can specify two values here: `30` (normal probing) and `10` (fast probing). The default value is `30`.
    #[builder(into, default)]
    #[serde(rename = "intervalInSeconds")]
    pub r#interval_in_seconds: Box<Option<i32>>,
    /// The path used by the monitoring checks. Required when `protocol` is set to `HTTP` or `HTTPS` - cannot be set when `protocol` is set to `TCP`.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// The port number used by the monitoring checks.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    /// The protocol used by the monitoring checks, supported values are `HTTP`, `HTTPS` and `TCP`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    /// The amount of time the Traffic Manager probing agent should wait before considering that check a failure when a health check probe is sent to the endpoint. If `interval_in_seconds` is set to `30`, then `timeout_in_seconds` can be between `5` and `10`. The default value is `10`. If `interval_in_seconds` is set to `10`, then valid values are between `5` and `9` and `timeout_in_seconds` is required.
    #[builder(into, default)]
    #[serde(rename = "timeoutInSeconds")]
    pub r#timeout_in_seconds: Box<Option<i32>>,
    /// The number of failures a Traffic Manager probing agent tolerates before marking that endpoint as unhealthy. Valid values are between `0` and `9`. The default value is `3`
    #[builder(into, default)]
    #[serde(rename = "toleratedNumberOfFailures")]
    pub r#tolerated_number_of_failures: Box<Option<i32>>,
}
